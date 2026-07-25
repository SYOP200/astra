FROM SYOP200/astra:0.4.0

COPY . /src/astra

RUN astra /src/SYOP200/bin/install --offline --noninteractive --yes
