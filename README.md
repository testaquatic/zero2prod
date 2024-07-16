# zero2pord

"제로부터 시작하는 러스트 백엔드 프로그래밍"( https://github.com/LukeMathWalker/zero-to-production ) 연습용 저장소

## 참고

- sqlx 오프라인 모드  
  `cargo sqlx prepare -- --lib`

- 도커 이미지 생성은 아래 명령을 이용한다.  
  `docker build --tag zero2prod --file Dockerfile .`

- 빌드한 도커 이미지 실행  
  `docker run -p 8000:8000 zero2prod`

- /healthcheck 엔드포인트 확인  
  `curl http://127.0.0.1:8000/health_check -v`
  => 200 OK

- /subscriptions 엔드포인트 확인  
  `curl --request POST --data 'email=thomas_mann@hotmail.com&name=Tom' --verbose http://127.0.0.1:8000/subscriptions`

- `TEST_LOG` 를 `true`로 설정하면 테스트 할 때 로그를 출력할 수 있다.  
  bunyan은 `cargo install bunyan`으로 설치할 수 있다.  
  `TEST_LOG=true cargo test health_check_works | bunyan`
