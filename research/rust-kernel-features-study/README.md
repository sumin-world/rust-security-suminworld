# Rust for Linux 핵심 기능 학습 프로젝트 🦀

> Rust로 리눅스 커널 개발하기 위한 핵심 언어 기능들을 학습하는 저장소입니다.

## 📚 학습 목표

이 프로젝트는 LWN.net 기사 "[Upcoming Rust language features for kernel development](https://lwn.net/Articles/1039073/)"를 바탕으로 다음 3가지 핵심 개념을 학습합니다:

1. **Field Projections** (필드 투영)
2. **In-place Initialization** (제자리 초기화)
3. **Arbitrary Self Types** (임의 자기 타입)

## 🚀 시작하기

### 사전 요구사항

```bash
# Rust 설치 (최신 stable 버전)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 설치 확인
rustc --version
cargo --version
```

### 프로젝트 실행

```bash
# 저장소 클론
git clone https://github.com/sumin-world/rust-kernel-features-study.git
cd rust-kernel-features-study

# 예제 실행
cargo run --example field_projection
cargo run --example inplace_init
cargo run --example smart_pointers
```

## 📖 학습 가이드

### 1단계: 기본 개념 이해하기

먼저 `docs/` 폴더의 문서들을 읽어보세요:

- `01_field_projection.md` - 필드 투영의 필요성
- `02_inplace_init.md` - 제자리 초기화가 왜 필요한가
- `03_smart_pointers.md` - 스마트 포인터와 self types

### 2단계: 현재 Rust의 한계 체험하기

`src/examples/limitations.rs`에서 현재 Rust의 제약사항을 확인해보세요:

```bash
cargo run --example limitations
```

### 3단계: 실습 예제 따라하기

각 기능별 예제를 실행하고 코드를 분석해보세요:

```bash
# Field Projection 예제
cargo run --example field_projection

# In-place Initialization 예제
cargo run --example inplace_init

# Smart Pointer 예제
cargo run --example smart_pointers
```

### 4단계: 도전 과제

`exercises/` 폴더의 연습문제를 풀어보세요!

## 📂 프로젝트 구조

```
rust-kernel-features-study/
├── src/
│   ├── lib.rs                 # 공통 유틸리티
│   └── examples/
│       ├── field_projection.rs    # 필드 투영 예제
│       ├── inplace_init.rs        # 제자리 초기화 예제
│       ├── smart_pointers.rs      # 스마트 포인터 예제
│       └── limitations.rs         # 현재 한계 데모
├── docs/
│   ├── 01_field_projection.md
│   ├── 02_inplace_init.md
│   └── 03_smart_pointers.md
├── exercises/
│   ├── exercise1.rs
│   ├── exercise2.rs
│   └── solutions/
└── README.md
```

## 🎯 주요 예제 설명

### 1. Field Projection (필드 투영)

**문제**: 구조체 포인터에서 특정 필드만 가리키는 포인터를 안전하게 만들기

```rust
struct Data {
    x: i32,
    y: String,
}

// 일반 참조는 쉬움
fn project_ref(data: &Data) -> &i32 {
    &data.x
}

// 하지만 커스텀 스마트 포인터에서는?
// 현재로서는 어려움!
```

**실습**: `examples/field_projection.rs` 참고

---

### 2. In-place Initialization (제자리 초기화)

**문제**: 큰 구조체를 스택에서 힙으로 이동할 때 오버헤드 발생

```rust
// ❌ 문제: 스택 오버플로우 위험
struct BigStruct {
    data: [u8; 100_000],  // 100KB!
}

let big = BigStruct { data: [0; 100_000] };  // 스택에 할당
let boxed = Box::new(big);  // 힙으로 복사 (추가 오버헤드)

// ✅ 이상적: 처음부터 힙에 생성
let boxed = Box::new_uninit();
// 직접 초기화...
```

**실습**: `examples/inplace_init.rs` 참고

---

### 3. Arbitrary Self Types (임의 자기 타입)

**문제**: 스마트 포인터로 메서드 호출이 불편함

```rust
use std::pin::Pin;

struct MyStruct {
    data: i32,
}

impl MyStruct {
    // ✅ 일반 참조는 가능
    fn regular_method(&self) {
        println!("{}", self.data);
    }
    
    // ❌ Pin<&mut Self>는 현재 불가능 (미래 기능)
    // fn pinned_method(self: Pin<&mut Self>) { }
}
```

**실습**: `examples/smart_pointers.rs` 참고

## 🔧 추가 학습 자료

### 공식 문서
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Reference - Pin](https://doc.rust-lang.org/std/pin/)
- [Rust Nomicon (Unsafe Rust)](https://doc.rust-lang.org/nomicon/)

### Rust for Linux 관련
- [Rust for Linux 공식 사이트](https://rust-for-linux.com/)
- [Linux Kernel 문서](https://docs.kernel.org/)
- [Field Projection RFC](https://github.com/rust-lang/rust/pull/146307)

### 유튜브 강의
- [Rust for Linux 소개](https://www.youtube.com/results?search_query=rust+for+linux)
- [Jon Gjengset의 Rust 스트림](https://www.youtube.com/@jonhoo)

## 🤝 기여하기

버그를 발견하거나 개선 아이디어가 있다면 이슈를 열어주세요!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 라이선스

MIT License - 자유롭게 사용하세요!

## 👤 작성자

- GitHub: [@sumin-world](https://github.com/sumin-world)
- Blog: [티스토리 블로그 링크]

## 🙏 감사의 말

이 프로젝트는 다음 자료들을 참고했습니다:
- LWN.net의 Daroc Alden 기사
- Rust for Linux 프로젝트 팀
- Benno Lossin, Xiangfei Ding의 발표

---

**⭐ 이 프로젝트가 도움이 되었다면 Star를 눌러주세요!**
