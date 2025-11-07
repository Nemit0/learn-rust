# 레슨 01 · 섹션 01 — 변수와 가변성

[English](section.md)

목표
- 변수 선언, 가변성(`mut`), 식과 문을 연습합니다.

가이드
- `let`은 기본적으로 불변 바인딩입니다. 변경하려면 `mut`를 붙입니다: `let mut x = 5; x = 10;`.
- 섀도잉은 같은 이름의 새 바인딩을 만드는 기법입니다: `let x = x + 1;` (타입 변경 가능). 반면 `mut`는 같은 바인딩/타입을 유지합니다.
- 식은 값을 반환하지만 문은 반환하지 않습니다. 블록의 마지막 식(세미콜론 없음)이 반환값이 됩니다.
- 기본 정수형: `i32`(기본), `u32`, `i64` 등.
- 함수에서 반환:
  - 암시적 꼬리 식: `x * 2`
  - 명시적 `return`: `return x * 2;`

빠른 데모
```rust
fn main() {
    let mut x = 5;   // 불변이 기본. 바꾸려면 `mut`
    x = 10;
    let y = x * 2;   // 식은 값을 반환
    println!("{x} {y}"); // 10 20
}
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

내장 치트시트(기본)
- 정수: `i32::MIN`, `i32::MAX`, `i32::abs(-3)`, `5.saturating_add(10)`, `5.wrapping_mul(3)`
- 불리언: `&&`, `||`, `!`
- 문자열:
  - `String::from("hi")`, `s.len()`, `s.is_empty()`, `s.push_str("!")`, `s.to_uppercase()`
  - `&str` 메서드: `"hi".len()`, `"hi rust".split_whitespace()`
- 벡터: `Vec::<i32>::new()`, `vec![1,2,3]`, `v.len()`, `v.push(4)`, `v.iter()`

과제
1) `make_ten()`이 `10`을 반환하도록 만드세요.
   - 가변 변수 선언 후 재할당해 보세요.
2) `double(x)`는 `2 * x`를 반환해야 합니다.

실행 방법
- 섹션 테스트: `cargo test -p lesson01_section01_variables`
- 전체 채점: `./scripts/grade.ps1` 또는 `bash ./scripts/grade.sh`

참고
- 정답 예시는 `answer.rs`에 있습니다(컴파일하지 않음).

