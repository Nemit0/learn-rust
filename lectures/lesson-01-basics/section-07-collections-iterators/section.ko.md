# 레슨 01 · 섹션 07 — 컬렉션과 이터레이터

[English](section.md)

요약
- 벡터, 슬라이스, 이터레이터 어댑터, `HashMap`을 사용합니다.

학습 목표
- 이터레이터 체인으로 변환하고 수집하기
- 슬라이스에서 짝수만 필터링하여 합계 구하기
- `HashMap::entry`로 단어 빈도 계산하기

사전 지식
- 메서드, 트레이트, 제네릭 기초

타임박스
- 15–20분

가이드
- 슬라이스 `&[T]`는 연속 메모리 뷰이며 `.iter()`로 순회합니다.
- 어댑터와 `collect::<Vec<_>>()`로 벡터를 만듭니다.
- 예시:
  - 제곱: `nums.iter().map(|n| n * n).collect::<Vec<_>>()`
  - 짝수 합: `nums.iter().copied().filter(|n| n % 2 == 0).sum::<i32>()`
- `HashMap` 카운팅 패턴: `*map.entry(key).or_insert(0) += 1;`
- 대소문자 무시: `to_lowercase()` 후 `split_whitespace()`

빠른 데모
```rust
use std::collections::HashMap;

fn square_all(nums: &[i32]) -> Vec<i32> { nums.iter().map(|n| n * n).collect() }
fn sum_of_evens(nums: &[i32]) -> i32 { nums.iter().copied().filter(|n| n % 2 == 0).sum() }
fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in text.to_lowercase().split_whitespace() { *m.entry(w.to_string()).or_insert(0) += 1; }
    m
}
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

검증
- 실행: `cargo test -p lesson01_section07_collections_iterators`

확장(선택)
- 영숫자만 남기도록 전처리를 추가해 보세요.

