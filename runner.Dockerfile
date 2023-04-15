FROM debian:stable-slim
# for development: 'tesseract-ocr-all' includes all testdata
# because the runner shouldn't be oversized, we only install the required 
#    packages for running the executable, which are the testdata/language files
RUN apt update && apt install -y tesseract-ocr-eng tesseract-ocr-deu
