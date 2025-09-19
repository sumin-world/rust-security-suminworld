🔎 Side-Channel Research — Flush+Reload (Cache) PoC

📋 개요
사이드 채널 공격은 연산의 시간, 전력 소비, 캐시 동작 등 부수적인 정보를 분석하여 비밀 정보를 유추하는 기법입니다. 이 저장소에는 캐시 기반의 대표적인 공격 기법인 Flush+Reload 개념 증명(PoC)이 포함되어 있습니다.
Flush+Reload는 공유 메모리의 캐시 상태를 조작하고 측정하여 다른 프로세스의 메모리 접근 패턴을 감지하는 기법입니다. 모든 실험 코드는 poCs/cache/ 디렉터리에 위치합니다.
📁 디렉터리 구조
poCs/cache/
├── victim_sim.c              # Victim: 특정 메모리 인덱스 반복 접근 (C)
├── flush_reload_attacker.c   # Attacker: clflush + rdtscp로 접근시간 측정 (C)
├── run.sh                    # 실험 자동화 스크립트 (start/collect/stop)
└── README.md                 # PoC 사용법 및 안전 주의사항
🚀 실행 방법
1. 기본 실행 예시
bash# 1) Victim 프로세스를 백그라운드로 실행
./poCs/cache/victim_sim & 
VICTIM_PID=$!

# 2) Attacker 실행하여 측정 데이터를 CSV로 저장
./poCs/cache/flush_reload_attacker > /tmp/flush_reload_data.csv

# 3) 실험 종료
kill $VICTIM_PID
2. 자동화 스크립트 사용
bash# run.sh를 사용한 전체 실험 자동화
./poCs/cache/run.sh
📊 데이터 형식 및 해석
CSV 출력 형식
csviter,cycles
0,158000
1,1000
2,1000
3,155000
4,1000
...
측정값 해석

작은 값 (약 1,000 cycles): 캐시 히트 - Victim이 해당 메모리에 접근함
큰 값 (100,000+ cycles): 캐시 미스 - Victim이 해당 메모리에 접근하지 않음 또는 인터럽트/컨텍스트 스위치 발생

📈 데이터 분석
간단한 통계 분석
bash# 샘플 수 확인
wc -l /tmp/flush_reload_data.csv

# 평균 계산
awk -F, 'NR>1{n++; sum+=$2} END{print "Samples:", n, "Mean:", sum/n}' /tmp/flush_reload_data.csv

# 샘플 수, 평균, 중앙값 계산
awk -F, 'NR>1{a[NR-1]=$2; n++; sum+=$2} END{print "n="n, "mean="sum/n; asort(a); print "median=" a[int(n/2)]; }' /tmp/flush_reload_data.csv
Python을 이용한 시각화
python# save as plot_flush_reload.py
import csv
import numpy as np
import matplotlib.pyplot as plt

xs = []
with open('/tmp/flush_reload_data.csv') as f:
    r = csv.reader(f)
    next(r)  # skip header
    for _, cycles in r:
        xs.append(int(cycles))

xs = np.array(xs)
plt.hist(xs, bins=200, log=True)
plt.xlabel('Cycles')
plt.ylabel('Count (log scale)')
plt.title('Flush+Reload Distribution')
plt.yscale('log')
plt.show()
🔍 결과 해석
이중 분포 (Bimodal Distribution)
데이터는 일반적으로 이중 분포를 보입니다:

저지연 피크: 캐시 히트 (Victim이 메모리 접근)
고지연 피크: 캐시 미스 (Victim이 메모리 미접근)

보안 영향
Flush+Reload 공격은 다음과 같은 실질적인 위협이 될 수 있습니다:

AES S-box 접근 추적: 암호화 키 추출 가능
RSA 키 비트 유추: 제곱-곱셈 패턴 분석
사용자 입력 패턴 추적: 키보드/마우스 입력 타이밍 분석

🛡️ 대응 및 완화 방안
소프트웨어 차원

상수 시간(Constant-time) 구현

비밀 정보에 독립적인 실행 시간 보장
분기문과 메모리 접근 패턴 균일화


메모리 접근 패턴 난독화

더미 접근 추가
블라인딩 기법 적용



하드웨어/시스템 차원

캐시 파티셔닝

Intel CAT (Cache Allocation Technology)
ARM 캐시 컬러링


프로세스 격리

코어 격리 (Core isolation)
전용 캐시 할당



⚖️ 법적 · 윤리적 고지
중요 공지
이 실험은 연구 및 교육 목적으로만 제공됩니다:

❌ 무단 테스트 금지: 허가되지 않은 시스템에 대한 테스트는 불법입니다
✅ 적법한 환경: 모든 실험은 법적 허가를 확보한 환경에서만 진행하세요
📋 책임 소재: 모든 사용에 따른 책임은 전적으로 사용자에게 있습니다

권장 실험 환경

개인 소유의 로컬 머신
격리된 가상 머신 (VM)
명시적 허가를 받은 테스트 환경
교육 기관의 승인된 실습 환경

📚 추가 리소스
관련 논문

Yuval Yarom and Katrina Falkner, "FLUSH+RELOAD: A High Resolution, Low Noise, L3 Cache Side-Channel Attack" (USENIX Security 2014)
Daniel Gruss et al., "Flush+Flush: A Fast and Stealthy Cache Attack" (DIMVA 2016)

추가 학습 자료

Cache-timing attacks on AES
Intel Software Guard Extensions (SGX) and Side-Channels


🔧 권장 추가 파일
리포지토리에 다음 파일들을 추가하는 것을 권장합니다:

poCs/cache/victim_sim.c — 간단한 C 구현 (주기적으로 특정 배열 인덱스 접근)
poCs/cache/flush_reload_attacker.c — clflush + rdtscp 측정, CSV 출력
poCs/cache/README.md — PoC 상세 사용법, 기대되는 출력, 안전·법적 안내
poCs/cache/run.sh — 실험 자동화 (백그라운드 시작, 데이터 수집, 종료)
docs/LEGAL_NOTICE.md — 법적 고지 전문

⚠️ 경고: 이 섹션의 코드와 실험은 교육 목적으로만 제공됩니다. 반드시 로컬 VM 또는 실험 전용 장비에서, 그리고 명시적 허가가 있는 환경에서만 실행하세요.
