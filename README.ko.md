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
- 저장된 스냅샷에서 `lectures/` 전체 트리 복원:
  - Windows: `./scripts/reset_libs.ps1`
  - macOS/Linux: `bash ./scripts/reset_libs.sh`
- Git 기준으로 되돌리기(원격 브랜치/커밋 기준):
  - Windows: `./scripts/reset_libs.ps1 -GitRef origin/main`
  - macOS/Linux: `bash ./scripts/reset_libs.sh --git-ref origin/main`

비고
- `answer.rs`는 컴파일하지 않습니다.
- 과제는 작고 결정적이어야 하며 테스트는 학습 목표를 정확히 표현해야 합니다.
- 리셋 스크립트는 이제 `lectures/` 전체 트리를 복원합니다(문서, 테스트, 라이브러리 등). 스냅샷 또는 git ref 기준으로 덮어쓰며, 스냅샷에 없는 추가 파일은 삭제하지 않습니다.
- 각 섹션 `section.md`에는 간단한 개념 설명과 실행 가능한 “Quick Demo”가 포함되어 있습니다.

채점 UI(선택 사항)
- 최소 데스크톱 UI가 `tools/grader_ui` 아래에 있습니다(워크스페이스에 포함하지 않아 채점 성능에 영향이 없습니다).
- 루트에서 실행:
  - Windows: `cargo run --manifest-path tools/grader_ui/Cargo.toml`
  - macOS/Linux: `cargo run --manifest-path tools/grader_ui/Cargo.toml`
- 기능:
  - `cargo metadata`로 `lectures/` 아래 패키지 자동 탐색
  - 선택 채점(`cargo test -p ...`), 전체 채점(워크스페이스 전체)
  - `scripts/reset_libs.*`를 통한 스냅샷/복원
  - Git ref(예: `origin/main`) 기준 복원

기본 사용법
- `Discover Packages`로 `lectures/**` 크레이트 목록을 가져옵니다.
- 일부 패키지를 선택하거나 `Select All`을 사용합니다.
- `Grade Selected`로 선택된 크레이트만 테스트하거나, `Grade All (workspace)`로 전체를 테스트합니다.
- 리셋:
  - `Snapshot Baseline`: `scripts/baseline/lectures/`에 스냅샷 생성/업데이트
  - `Restore Baseline`: 스냅샷을 `lectures/`로 복원
  - `Restore from Git Ref`: 입력한 Git ref 기준으로 `lectures/` 복원

참고
- UI는 하단 영역에 명령 출력과 종료 상태를 표시합니다.
- 설치된 `cargo`, Windows에선 PowerShell(`.ps1`), macOS/Linux에선 `bash`(`.sh`)를 사용해 스크립트를 실행합니다.
- UI 실행 시 워크스페이스 관련 오류가 보이면 루트 `Cargo.toml`에 `exclude = ["tools/grader_ui"]`가 포함되어 있는지 확인하세요.
