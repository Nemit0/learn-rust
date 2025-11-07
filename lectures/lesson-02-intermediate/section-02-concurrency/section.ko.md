# 레슨 02 · 섹션 02 — 동시성(스레드와 조인)

[English](section.md)

요약
- 스레드를 생성하고 조인하여 결과를 안전하게 합칩니다.

학습 목표
- 작업을 청크로 나누고 워커 스레드를 생성
- `JoinHandle`로 결과를 수집하고 결정적으로 합치기
- `n_threads == 0` 같은 엣지 케이스 처리

사전 지식
- 소유권/차용, 이터레이터 기초

타임박스
- 15–20분

가이드
- `thread::spawn(move || { ... })`로 워커를 시작합니다. `move`는 캡처 값을 스레드로 이동시킵니다.
- 항상 `handle.join().expect("thread panicked")`로 조인하여 결과를 회수합니다.
- `chunks(chunk_size)`로 입력을 균등하게 나누고 올림 나눗셈으로 `chunk_size`를 계산합니다.
- 수명 문제를 피하려면 스레드에 소유 데이터를 전달하세요(예: `chunk.to_vec()`).

빠른 데모
```rust
use std::thread;

fn main() {
    let data: Vec<i32> = (1..=10_000).collect();
    let mid = data.len()/2;
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();
    let h1 = thread::spawn(move || left.iter().copied().sum::<i32>());
    let h2 = thread::spawn(move || right.iter().copied().sum::<i32>());
    let total = h1.join().unwrap() + h2.join().unwrap();
    println!("sum={}", total);
}
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

검증
- 실행: `cargo test -p lesson02_section02_concurrency`

