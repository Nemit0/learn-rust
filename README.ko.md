# 러스트 학습 — 강의와 채점 도구

개요
- 작은 단위의 러스트 연습 문제들을 워크스페이스로 구성했습니다. 각 섹션은 빠르게 컴파일되고 테스트됩니다.
- 각 섹션에는 다음 파일이 포함됩니다:
  - `section.md` — 과제 설명과 목표, 빠른 데모
  - `src/lib.rs` — 학생이 채워넣을 불완전한 코드
  - `answer.rs` — 참고 정답(컴파일되지 않음)
  - `tests/` — 채점에 사용하는 테스트들

사전 준비
- 러스트 툴체인(rustup): `rustc --version` 확인
- Cargo(러스트와 함께 설치)

프로젝트 구조
- `lectures/lesson-01-basics/section-01-variables` 등 섹션별 크레이트
- `scripts/grade.ps1` — Windows용 파워셸 채점기
- `scripts/grade.sh` — macOS/Linux용 최소 채점기

워크플로우
1) `lectures/...` 아래에서 섹션을 고르고 `section.md`를 읽습니다.
2) `src/lib.rs`를 수정하여 과제를 풉니다.
3) 섹션별 테스트 실행:
   - `cargo test -p lesson01_section01_variables`
   - `cargo test -p lesson01_section02_functions`
4) 전체 채점:
   - Windows: `./scripts/grade.ps1`
   - macOS/Linux: `bash ./scripts/grade.sh`

초기 상태로 되돌리기
- 현재 시작 코드 스냅샷 저장(최초 1회):
  - Windows: `./scripts/reset_libs.ps1 -Init`
  - macOS/Linux: `bash ./scripts/reset_libs.sh --init`
- 저장된 스냅샷에서 모든 `lectures/**/src/lib.rs` 복원:
  - Windows: `./scripts/reset_libs.ps1`
  - macOS/Linux: `bash ./scripts/reset_libs.sh`
- Git 기준으로 되돌리기(원격 브랜치/커밋 기준):
  - Windows: `./scripts/reset_libs.ps1 -GitRef origin/main`
  - macOS/Linux: `bash ./scripts/reset_libs.sh --git-ref origin/main`

비고
- `answer.rs`는 컴파일하지 않습니다.
- 과제는 작고 결정적이어야 하며 테스트는 학습 목표를 정확히 표현해야 합니다.
- 각 섹션 `section.md`에는 간단한 개념 설명과 실행 가능한 “Quick Demo”가 포함되어 있습니다.

