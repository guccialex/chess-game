FROM python:latest


WORKDIR /app

COPY ./yew-chess-game/dist .

RUN ls

RUN pwd


CMD python3 -m http.server