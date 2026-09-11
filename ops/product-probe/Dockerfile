FROM python:3.12-slim
RUN pip install --no-cache-dir rfc8785==0.1.4
WORKDIR /probe
COPY ops/product-probe/runner.py /probe/runner.py
USER 65532:65532
ENTRYPOINT ["python", "/probe/runner.py"]
