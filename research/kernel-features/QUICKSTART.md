# 빠른 시작 가이드 🚀

## 5분 만에 시작하기

### 1단계: 저장소 클론

```bash
git clone https://github.com/sumin-world/rust-kernel-features-study.git
cd rust-kernel-features-study
```

### 2단계: Rust 설치 확인

```bash
rustc --version
cargo --version
```

Rust가 없다면:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3단계: 예제 실행

```bash
# 전체 한계 종합 (추천!)
cargo run --example limitations

# 개별 예제
cargo run --example field_projection
cargo run --example inplace_init
cargo run --example smart_pointers
```

### 4단계: 문서 읽기

```bash
# 브라우저로 docs/ 폴더의 마크다운 파일 열기
```

## 학습 순서

1. **기초 개념** (30분)
   - `docs/01_field_projection.md`
   - `docs/02_inplace_init.md`
   - `docs/03_smart_pointers.md`

2. **종합 데모** (15분)
   ```bash
   cargo run --example limitations
   ```

3. **개별 예제** (각 10분)
   ```bash
   cargo run --example field_projection
   cargo run --example inplace_init
   cargo run --example smart_pointers
   ```

4. **코드 분석** (30분)
   - `src/examples/` 폴더의 코드 읽기
   - 주석 따라가기

## 다음 단계

- [ ] 티스토리 블로그 글 읽기
- [ ] 예제 코드 수정해보기
- [ ] LWN.net 원문 읽기
- [ ] Rust for Linux 프로젝트 탐색

## 문제 해결

### Rust 설치 문제
```bash
# Rust 재설치
rustup update
```

### 빌드 오류
```bash
# 캐시 정리
cargo clean
cargo build
```

### 실행 오류
```bash
# 자세한 로그
RUST_LOG=debug cargo run --example limitations
```

## 도움 받기

- GitHub Issues: [이슈 생성](https://github.com/sumin-world/rust-kernel-features-study/issues)
- 티스토리 블로그 댓글
- Rust Korea 커뮤니티

즐거운 학습 되세요! 🦀
