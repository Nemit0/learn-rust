# 레슨 02 · 섹션 03 — 이터레이터(커스텀 + 어댑터)

[English](section.md)

요약
- 커스텀 `Iterator`를 만들고 어댑터로 조합해 결과를 수집합니다.

학습 목표
- 상태를 가진 타입에 `Iterator` 구현
- `.collect()`, `.take()` 등 어댑터 사용
- 길이와 내용 검증을 위한 테스트 작성

사전 지식
- 구조체/메서드, 이터레이터와 클로저 기초

타임박스
- 10–15분

가이드
- `next(&mut self) -> Option<Item>`을 구현하고, 끝나면 `None`을 반환합니다.
- 각 `next()` 호출에서 상태를 원자적으로 갱신하세요.
- `.map`, `.take`, `.collect::<Vec<_>>()`로 변환/수집합니다.

빠른 데모
```rust
#[derive(Clone)]
struct Fib { curr: u64, next: u64, left: usize }
impl Fib { fn new(n: usize) -> Self { Self { curr: 0, next: 1, left: n } } }
impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.left == 0 { return None; }
        let out = self.curr;
        let new_next = self.curr + self.next;
        self.curr = self.next; self.next = new_next; self.left -= 1; Some(out)
    }
}
fn main() { println!("{:?}", Fib::new(7).collect::<Vec<_>>()); }
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

검증
- 실행: `cargo test -p lesson02_section03_iterators_advanced`

