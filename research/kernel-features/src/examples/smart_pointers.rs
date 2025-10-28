//! Arbitrary Self Types (임의 자기 타입) 예제
//! 
//! 커스텀 스마트 포인터로 메서드를 호출하는 예제입니다.
//! 현재의 제약과 미래의 가능성을 탐구합니다.

use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;

/// 간단한 데이터 구조체
#[derive(Debug)]
struct Data {
    value: i32,
    name: String,
}

impl Data {
    /// ✅ 일반 참조로 받는 메서드 (현재 가능)
    fn print_value(&self) {
        println!("Value: {}", self.value);
    }
    
    /// ✅ 가변 참조로 받는 메서드 (현재 가능)
    fn increment(&mut self) {
        self.value += 1;
    }
    
    /// ✅ 소유권을 받는 메서드 (현재 가능)
    fn consume(self) -> i32 {
        self.value
    }
}

/// 커스텀 스마트 포인터 #1: 참조 카운팅 포인터
struct MyRc<T> {
    inner: Rc<T>,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        Self {
            inner: Rc::new(value),
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

/// 커스텀 스마트 포인터 #2: 안전한 포인터
struct SafePtr<T> {
    inner: Box<T>,
    is_valid: bool,
}

impl<T> SafePtr<T> {
    fn new(value: T) -> Self {
        Self {
            inner: Box::new(value),
            is_valid: true,
        }
    }
    
    fn invalidate(&mut self) {
        self.is_valid = false;
    }
    
    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl<T> Deref for SafePtr<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        assert!(self.is_valid, "Attempted to access invalidated pointer!");
        &self.inner
    }
}

/// Pin과 함께 사용하는 타입
struct PinnedData {
    value: i32,
}

impl PinnedData {
    /// ❌ 현재 불가능: Pin<&mut Self>를 self로 받기
    /// 
    /// ```rust,ignore
    /// fn modify_pinned(self: Pin<&mut Self>) {
    ///     // Pin 상태에서 안전하게 수정
    /// }
    /// ```
    
    /// ✅ 현재의 해결책: 일반 함수로 작성
    fn modify_pinned(pinned: Pin<&mut Self>) {
        unsafe {
            Pin::get_unchecked_mut(pinned).value += 1;
        }
    }
}

/// Arc를 self로 받고 싶은 경우
struct SharedData {
    id: u32,
}

impl SharedData {
    /// ❌ 현재 불가능: Arc<Self>를 self로 받기
    /// 
    /// ```rust,ignore
    /// fn with_arc(self: Arc<Self>) -> Arc<Self> {
    ///     println!("ID: {}", self.id);
    ///     self
    /// }
    /// ```
    
    /// ✅ 현재의 해결책: Arc를 매개변수로 받기
    fn with_arc_workaround(arc: Arc<Self>) -> Arc<Self> {
        println!("ID: {}", arc.id);
        arc
    }
}

/// 현재 방식의 문제점 시연
fn demonstrate_current_limitations() {
    println!("\n=== 현재 방식의 한계 ===");
    
    let data = Data {
        value: 42,
        name: "test".to_string(),
    };
    
    // 1. 일반 참조는 .method() 문법 사용 가능
    println!("\n1. 일반 참조:");
    data.print_value();  // ✅ 작동
    
    // 2. Box도 Deref 덕분에 가능
    println!("\n2. Box:");
    let boxed = Box::new(Data {
        value: 100,
        name: "boxed".to_string(),
    });
    boxed.print_value();  // ✅ 작동 (Deref를 통해)
    
    // 3. 커스텀 스마트 포인터
    println!("\n3. 커스텀 스마트 포인터 (MyRc):");
    let rc_data = MyRc::new(Data {
        value: 200,
        name: "rc".to_string(),
    });
    rc_data.print_value();  // ✅ Deref 덕분에 작동
    
    // 하지만...
    println!("\n4. 문제: 메서드에서 스마트 포인터 자체를 받을 수 없음");
    println!("   예: fn method(self: MyRc<Self>) {{ }} // ❌ 불가능");
}

/// 미래 문법 시연 (주석으로)
fn demonstrate_future_syntax() {
    println!("\n=== 미래 문법 (Arbitrary Self Types) ===");
    
    println!("\n현재는 다음과 같은 코드가 불가능합니다:");
    println!(r#"
impl Data {{
    fn with_rc(self: Rc<Self>) {{
        println!("Value: {{}}", self.value);
    }}
    
    fn with_arc(self: Arc<Self>) -> Arc<Self> {{
        self
    }}
    
    fn with_pin(self: Pin<&mut Self>) {{
        // Pin 상태 유지하면서 수정
    }}
    
    fn with_custom(self: MyRc<Self>) {{
        // 커스텀 포인터로 직접 받기
    }}
}}
"#);
    
    println!("하지만 Arbitrary Self Types가 추가되면 가능해집니다!");
}

/// Pin 사용 시나리오
fn demonstrate_pin_scenario() {
    println!("\n=== Pin 시나리오 ===");
    
    let mut data = PinnedData { value: 0 };
    let mut pinned = Pin::new(&mut data);
    
    println!("초기 값: {}", pinned.value);
    
    // 현재 방식: 함수로 전달
    PinnedData::modify_pinned(pinned.as_mut());
    println!("수정 후: {}", pinned.value);
    
    println!("\n미래에는 이렇게 가능:");
    println!("  pinned.modify_pinned(); // Pin<&mut Self>를 self로");
}

/// Arc 공유 시나리오
fn demonstrate_arc_scenario() {
    println!("\n=== Arc 공유 시나리오 ===");
    
    let shared = Arc::new(SharedData { id: 42 });
    
    println!("현재 방식:");
    let shared2 = SharedData::with_arc_workaround(Arc::clone(&shared));
    println!("Arc 강한 참조 개수: {}", Arc::strong_count(&shared));
    drop(shared2);
    
    println!("\n미래 방식 (더 자연스러움):");
    println!("  let result = shared.with_arc();");
    println!("  // Arc<Self>를 self로 직접 받음");
}

/// 실제 커널 사용 사례
fn real_world_kernel_example() {
    println!("\n=== 실제 커널 사용 사례 ===");
    
    println!("리눅스 커널에서 흔한 패턴:");
    println!(r#"
// 장치 드라이버
struct Device {{
    name: String,
    // ...
}}

impl Device {{
    // ❌ 현재: 불가능
    // fn register(self: Arc<Self>) -> Result<(), Error> {{
    //     // Arc로 여러 곳에서 공유해야 함
    // }}
    
    // ❌ 현재: 불가능
    // fn unregister(self: Pin<Arc<Self>>) {{
    //     // Pin + Arc 조합
    // }}
}}

// 대신 이렇게 해야 함 (불편!)
fn register_device(dev: Arc<Device>) -> Result<(), Error> {{
    // ...
}}
"#);
    
    println!("Arbitrary Self Types가 있으면:");
    println!("  ✅ 더 자연스러운 API");
    println!("  ✅ 타입 안전성 유지");
    println!("  ✅ 코드 가독성 향상");
}

/// Receiver trait 개념
fn demonstrate_receiver_trait() {
    println!("\n=== Receiver Trait 개념 ===");
    
    println!("Arbitrary Self Types 구현 방법:");
    println!(r#"
// Receiver trait (컴파일러에서 제공)
trait Receiver {{
    type Target;
}}

// 기본 포인터 타입들은 자동으로 구현
impl<T> Receiver for &T {{ ... }}
impl<T> Receiver for &mut T {{ ... }}
impl<T> Receiver for Box<T> {{ ... }}
impl<T> Receiver for Rc<T> {{ ... }}
impl<T> Receiver for Arc<T> {{ ... }}
impl<T> Receiver for Pin<&T> {{ ... }}

// 커스텀 포인터도 구현 가능
impl<T> Receiver for MyRc<T> {{
    type Target = T;
}}
"#);
    
    println!("이렇게 하면 점진적으로 채택 가능!");
}

fn main() {
    println!("=== Arbitrary Self Types 예제 ===");
    
    // 1. 현재 한계 시연
    demonstrate_current_limitations();
    
    // 2. 미래 문법
    demonstrate_future_syntax();
    
    // 3. Pin 시나리오
    demonstrate_pin_scenario();
    
    // 4. Arc 시나리오
    demonstrate_arc_scenario();
    
    // 5. 실제 커널 사례
    real_world_kernel_example();
    
    // 6. Receiver trait
    demonstrate_receiver_trait();
    
    // 결론
    println!("\n=== 결론 ===");
    println!("Arbitrary Self Types가 추가되면:");
    println!("  ✅ 스마트 포인터를 self로 받을 수 있음");
    println!("  ✅ 더 자연스러운 메서드 체이닝");
    println!("  ✅ Pin + Arc 같은 조합 지원");
    println!("  ✅ 커널 코드 작성이 훨씬 쉬워짐");
    println!("\n현재 상태:");
    println!("  🚧 구현 진행 중");
    println!("  🚧 1년 내 완료 가능성 있음");
    println!("  🚧 Receiver trait 방식으로 설계");
}
