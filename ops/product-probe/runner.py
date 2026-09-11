#!/usr/bin/env python3
"""Product-owned assertions; protocol and evidence boundary: CONTRACT.md."""
import datetime
import hashlib
import json
import os
import re
import signal
import sys
import time
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from contextlib import contextmanager
from urllib.parse import urlencode, urlsplit

import rfc8785

IDS = tuple('mark.' + s + '.v1' for s in (
    'svg-grammar', 'svg-escaping', 'deterministic-paint', 'badge-equivalence',
    'conditional-cache', 'catalog'))
PREFIX = 'DELIVERY_PRODUCT_PROBE_ASSERTION_STATUS_'
SVG = '{http://www.w3.org/2000/svg}'


def timestamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat().replace('+00:00', 'Z')


def parse_context(raw, digest):
    if len(raw.encode()) > 65536:
        raise ValueError('context too large')
    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise ValueError('duplicate key')
            result[key] = value
        return result
    c = json.loads(raw, object_pairs_hook=pairs)
    if 'sha256:' + hashlib.sha256(rfc8785.dumps(c)).hexdigest() != digest:
        raise ValueError('digest mismatch')
    def string(obj, key):
        if not isinstance(obj.get(key), str) or not obj[key]:
            raise ValueError('missing binding')
        return obj[key]
    def uint64(obj, key):
        value = string(obj, key)
        if not re.fullmatch(r'[1-9][0-9]{0,19}', value) or int(value) > 2**64-1:
            raise ValueError('invalid generation')
    def sha(obj, key):
        if not re.fullmatch(r'sha256:[0-9a-f]{64}', string(obj, key)):
            raise ValueError('invalid digest')
    for key in ('organizationId', 'projectId', 'environmentId'):
        string(c, key)
    release = c['release']
    for key in ('releaseId', 'decisionId'):
        string(release, key)
    uint64(release, 'selectionGeneration')
    if type(release['rolloutAttempt']) is not int or not 1 <= release['rolloutAttempt'] <= 2**32-1:
        raise ValueError('invalid rollout')
    if release['stage'] != 'DELIVERY_RELEASE_EXECUTION_STAGE_POST_DEPLOYMENT_PROBE':
        raise ValueError('wrong stage')
    intent = c['intent']
    if intent['contractId'] != 'mark.public-svg.v1':
        raise ValueError('wrong contract')
    sha(intent, 'runnerImageDigest')
    string(intent, 'policyRevision')
    ids = intent['requiredAssertionIds']
    if not isinstance(ids, list) or len(ids) != len(IDS) or set(ids) != set(IDS):
        raise ValueError('incomplete or unknown assertion set')
    targets = intent['targets']
    if not isinstance(targets, list) or len(targets) != 1:
        raise ValueError('one selected target required')
    target = targets[0]
    for key in ('memberId', 'revisionUid', 'sourceRepositoryId'):
        string(target, key)
    uint64(target, 'resourceGeneration')
    sha(target, 'runtimeObservationDigest')
    if not re.fullmatch(r'[0-9a-f]{40}|[0-9a-f]{64}', string(target, 'sourceCommitSha')):
        raise ValueError('invalid source commit')
    origin = string(target, 'origin')
    if any(ch.isspace() or ord(ch) < 33 or ord(ch) == 127 for ch in origin):
        raise ValueError('invalid origin')
    url = urlsplit(origin)
    if (url.scheme != 'https' or not url.hostname or url.username is not None
            or url.password is not None or url.query or url.fragment or url.path not in ('', '/')
            or (url.port is not None and not 1 <= url.port <= 65535)):
        raise ValueError('invalid selected origin')
    return ids, origin.rstrip('/')


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, *args, **kwargs):
        raise ValueError('redirect forbidden')


@contextmanager
def deadline(seconds):
    started = time.monotonic()
    previous_handler = signal.getsignal(signal.SIGALRM)
    def expire(*_):
        raise TimeoutError('deadline')
    signal.signal(signal.SIGALRM, expire)
    previous = signal.setitimer(signal.ITIMER_REAL, seconds)
    if previous[0] > 0:
        signal.setitimer(signal.ITIMER_REAL, min(seconds, previous[0]))
    try:
        yield
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
        signal.signal(signal.SIGALRM, previous_handler)
        if previous[0] > 0:
            signal.setitimer(signal.ITIMER_REAL, max(.000001, previous[0] - (time.monotonic() - started)), previous[1])


def get(origin, path, headers=None):
    with deadline(5):
        request = urllib.request.Request(origin + path, headers=headers or {})
        opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())
        try:
            response = opener.open(request, timeout=2)
        except urllib.error.HTTPError as exc:
            if exc.code != 304:
                raise
            response = exc
        with response:
            raw = response.read(1024 * 1024 + 1)
            if len(raw) > 1024 * 1024:
                raise ValueError('response too large')
            return response.code, response.headers, raw


def mark(form, **query):
    return '/api/v1/mark/' + form + '?' + urlencode({'credit': '0', 'animation': 'none', **query})


def svg(origin, path):
    status, headers, raw = get(origin, path)
    assert status == 200 and headers.get_content_type() == 'image/svg+xml'
    root = ET.fromstring(raw)
    assert root.tag == SVG + 'svg'
    return headers, raw, root


def grammar(origin):
    cases = [('hero', {'type': 'soft', 'text': 'Probe hero', 'width': 640, 'height': 240}, 'Probe hero'),
             ('pill', {'label': 'build', 'message': 'passing'}, 'passing'),
             ('strip', {'icons': 'rust,ts'}, 'rust'),
             ('profile', {'text': 'Ada Lovelace'}, 'Ada Lovelace'),
             ('deploy', {'service': 'mark'}, 'mark')]
    for form, query, expected in cases:
        _, _, root = svg(origin, mark(form, **query))
        assert expected in ''.join(root.itertext())
        if form == 'hero':
            assert root.attrib['width'] == '640' and root.attrib['height'] == '240'


def escaping(origin):
    hostile = '<script>x</script>&"'
    path = mark('hero', type='soft', text=hostile, fontColor='" onload="alert(7)')
    headers, raw, root = svg(origin, path)
    assert hostile in ''.join(root.itertext())
    assert b'&lt;script&gt;' in raw and b'&amp;' in raw
    for node in root.iter():
        assert node.tag != SVG + 'script'
        assert not any(key.lower().startswith('on') for key in node.attrib)
    assert "script-src 'none'" in headers.get('content-security-policy', '')
    assert headers.get('x-content-type-options') == 'nosniff'


def deterministic(origin):
    path = mark('hero', type='aurora', text='Ship your release')
    ha, a, _ = svg(origin, path)
    hb, b, _ = svg(origin, path)
    assert a == b and ha['etag'] == hb['etag']
    _, fallback, _ = svg(origin, path + '&theme=unknown-probe-theme')
    _, neon, _ = svg(origin, path + '&theme=neon')
    assert fallback == a and neon != a


def badge(origin):
    opts = {'credit': '0', 'animation': 'none', 'style': 'pill', 'font': 'mono', 'labelColor': '123456'}
    _, shorthand, _ = svg(origin, '/badge/build-passing-green?' + urlencode(opts))
    _, canonical, _ = svg(origin, '/api/v1/mark/pill?' + urlencode({**opts, 'label': 'build', 'message': 'passing', 'color': 'green'}))
    assert shorthand == canonical


def cache(origin):
    path = mark('hero', type='soft', text='Cache probe')
    headers, _, _ = svg(origin, path)
    etag = headers.get('etag', '')
    assert re.fullmatch(r'"[^"]+"', etag)
    for name in ('cache-control', 'cdn-cache-control', 'cloudflare-cdn-cache-control'):
        value = headers.get(name, '')
        assert 'public' in value and 'max-age=31536000' in value and 'immutable' in value
    status, conditional, body = get(origin, path, {'If-None-Match': etag})
    assert status == 304 and body == b'' and conditional.get('etag') == etag
    for name in ('cache-control', 'cdn-cache-control', 'cloudflare-cdn-cache-control'):
        assert conditional.get(name) == headers.get(name)
    changed, _, _ = svg(origin, mark('hero', type='soft', text='Changed content'))
    assert changed['etag'] != etag


def catalog(origin):
    status, headers, raw = get(origin, '/api/v1/catalog')
    assert status == 200 and headers.get_content_type() == 'application/json'
    data = json.loads(raw)
    assert data['forms'] == ['hero', 'pill', 'strip', 'profile', 'deploy']
    assert data['limits'] == {'text': 500, 'desc': 240, 'lines': 8, 'pill_label': 80,
                              'pill_message': 120, 'strip_icons': 60, 'deploy_service': 40}
    for key in ('themes', 'icons', 'art_types', 'layouts', 'fonts'):
        assert isinstance(data[key], list) and data[key]
    assert not {'kyle', 'sylphx', 'cubeage', 'epiow', 'ozyrix'} & set(data['themes'] + data['icons'])
    icon = data['icons'][0]
    _, _, root = svg(origin, mark('strip', icons=icon))
    assert any(node.text == icon for node in root.iter(SVG + 'title'))
    assert '>?</text>' not in ET.tostring(root, encoding='unicode')
    svg(origin, mark('hero', type=data['art_types'][0], text='Catalog art'))


CHECKS = dict(zip(IDS, (grammar, escaping, deterministic, badge, cache, catalog)))


def run(origin, ids, digest, output='/dev/termination-log'):
    started = timestamp()
    assertions = [{'id': i, 'status': PREFIX + 'SKIPPED'} for i in ids]
    try:
        with deadline(50):
            for assertion in assertions:
                assertion['status'] = PREFIX + 'FAILED'
                CHECKS[assertion['id']](origin)
                assertion['status'] = PREFIX + 'PASSED'
    except Exception:
        print('Mark product assertion failed', file=sys.stderr)
    result = json.dumps({'contextDigest': digest, 'assertions': assertions,
                         'startedAt': started, 'finishedAt': timestamp()}, separators=(',', ':'))
    if len(result.encode()) > 4096:
        raise ValueError('result too large')
    with open(output, 'w') as target:
        target.write(result)
    return 0 if all(a['status'] == PREFIX + 'PASSED' for a in assertions) else 1


def main():
    try:
        digest = os.environ['APPS_PRODUCT_PROBE_CONTEXT_DIGEST']
        ids, origin = parse_context(os.environ['APPS_PRODUCT_PROBE_CONTEXT'], digest)
        return run(origin, ids, digest)
    except Exception:
        print('invalid Mark product probe input or output', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
