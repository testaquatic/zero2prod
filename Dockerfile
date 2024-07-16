# 기본 이미지로 최신 러스트 stable 릴리즈를 사용한다.
# Builder 단계
FROM rust:latest AS builder

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

# Runtime 단계
FROM debian:stable-slim AS runtime

WORKDIR /app
# OpenSSL을 설치한다. - 일부 디펜던시에 의해 동적으로 링크된다.
# ca-certificates를 설치한다. - HTTPS 연결을 수립할 때 TLS 인증 검증에 필요하다.
RUN apt update -y \
    && apt install -y --no-install-recommends openssl ca-certificates \
    # 클린업
    && apt autoremove -y \
    && apt clean -y \
    && rm -rf /var/lib/apt/lists/*
# 컴파일된 바이너리를 builder 환경에서 runtime 환경으로 복사한다.
COPY --from=builder /app/target/release/zero2prod zero2prod
# runtime에서의 구성 파일이 필요하다!
COPY configuration configuration
ENV APP_ENVIRONMENT=production
# `docker run`이 실행되면 바이너리를 구동한다.
ENTRYPOINT [ "./zero2prod" ]