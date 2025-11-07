# 레슨 02 · 섹션 01 — 스마트 포인터(Box, Rc)

[English](section.md)

요약
- `Box`로 재귀 자료구조를 만들고 `Rc`로 공유 소유권을 연습합니다.

학습 목표
- `Box<T>`로 재귀 타입 구성, 열거형 패턴 매칭
- 단일 스레드 공유 소유권 `Rc<T>` 사용 및 strong count 확인
- 재귀 순회로 값 수집

사전 지식
- 레슨 01: 열거형/패턴 매칭, 기본 소유권/차용

타임박스
- 15분

가이드
- `Box<T>`는 힙에 데이터를 저장해 재귀 타입(연결 리스트 등)을 만들 수 있습니다.
- cons 노드: `List::Cons(value, Box::new(tail))`
- 재귀 순회: `match Cons(h, t)`에서 꼬리로 내려가며 처리
- `Rc<T>`는 단일 스레드에서 공유 소유권을 제공합니다. `Rc::clone(&rc)`로 참조 수 증가.
- `Rc::strong_count(&rc)`로 카운트를 확인할 수 있습니다.

빠른 데모
```rust
use std::rc::Rc;

enum List { Cons(i32, Box<List>), Nil }
use List::*;

fn len(lst: &List) -> usize { match lst { Nil => 0, Cons(_, t) => 1 + len(t) } }

fn main() {
    let lst = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("len={}", len(&lst));
    let rc = Rc::new(42);
    let a = Rc::clone(&rc);
    println!("count={}", Rc::strong_count(&rc));
    drop(a);
    println!("count={}", Rc::strong_count(&rc));
}
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

검증
- 실행: `cargo test -p lesson02_section01_smart_pointers`

