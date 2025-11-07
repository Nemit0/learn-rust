# 레슨 03 · 섹션 02 — 선언적 매크로(고급)

[English](section.md)

요약
- 반복 패턴과 후행 콤마를 지원하는 데이터 생성 매크로를 작성합니다.

학습 목표
- 입력 표현식에서 소유 데이터 구조로 확장
- 반복 구문 `$( ... ),*` 와 선택적 후행 콤마 `$(,)?` 사용
- 위생(hygiene)을 지키며 간결한 매크로 작성

사전 지식
- 기본 `macro_rules!` 문법, 컬렉션과 문자열

타임박스
- 15분

가이드
- 쉼표로 구분된 입력을 `to_string()`으로 매핑해 `Vec<String>`을 만듭니다.
- `key => value` 쌍으로 `HashMap<String, i32>`를 구성합니다. 선택적 후행 콤마를 허용하세요.

빠른 데모
```rust
macro_rules! vec_of_strings { ($($x:expr),* $(,)?) => { vec![$($x.to_string()),*] }; }
macro_rules! make_map { ($($k:expr => $v:expr),* $(,)?) => {{
    let mut m = std::collections::HashMap::new();
    $( m.insert($k.to_string(), $v); )*
    m
}} }
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

시작 코드
- `src/lib.rs`의 `vec_of_strings!`, `make_map!`을 구현하세요.

검증
- 실행: `cargo test -p lesson03_section02_advanced_macros`

흔한 실수
- 후행 콤마를 허용하지 않음
- `$x:expr` 대신 `$x:ident`를 사용해 복잡한 표현식이 거부되는 경우

