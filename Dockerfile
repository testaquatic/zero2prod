# 이미지 빌드 방법
# docker build --tag zero2prod --file Dockerfile . 

# 기본 이미지로 최신 러스트 stable 릴리즈를 사용한다.
FROM rust:latest

# 작업 디렉터리를 `app`으로 변경한다.
# `app` 디렉터리가 존재하지 않는 경우 도커가 해당 폴더를 생성한다.
WORKDIR /app
# 구성을 연결하기 위한 필요한 시스템 디펜던시를 설치한다.
# RUN apt update &&
# 작업 환경의 모든 파일을 도커 이미지로 복사한다.
COPY . .
# sqlx의 오프라인 모드를 사용한다.
ENV SQLX_OFFLINE=true
# 바이너리를 빌드하자.
# release 프로파일을 사용한다.
RUN cargo build --release
ENV APP_ENVIRONMENT=production
# `docker run`이 실행되면 바이너리를 구동한다.
ENTRYPOINT [ "./target/release/zero2prod" ]