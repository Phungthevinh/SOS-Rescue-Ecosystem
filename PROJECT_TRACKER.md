# 🚨 BEACON - SOS Rescue Ecosystem
## Project Development Tracker

> **Tên dự án:** Beacon (SOS Rescue Ecosystem)  
> **Ngày bắt đầu:** 2026-06-17  
> **Công nghệ chính:** Flutter (UI) + Rust (Core Mobile & Backend)  
> **Mục tiêu:** Ứng dụng khẩn cấp cộng đồng - bấm SOS → gửi vị trí → thông báo người thân + cộng đồng lân cận  

---

## 📌 Trạng thái Tổng quan

| Phase | Mô tả | Trạng thái | Timeline dự kiến |
|-------|--------|------------|-------------------|
| Phase 0 | Foundation & Setup | 🔵 Đang tiến hành (~90%) | Tuần 1-2 |
| Phase 1 | Auth & Core SOS (MVP) | 🔵 Đang tiến hành (~10%) | Tháng 1-2 |
| Phase 2 | Community Radar & Battery Optimization | 🔲 Chưa bắt đầu | Tháng 3-5 |
| Phase 3 | BLE, Blackbox & Advanced Features | 🔲 Chưa bắt đầu | Tháng 6-7 |
| Phase 4 | Testing, Security & Release | 🔲 Chưa bắt đầu | Tháng 8 |

**Ký hiệu:** 🔲 Chưa bắt đầu | 🔵 Đang tiến hành | ✅ Hoàn thành | ❌ Bị chặn | ⏸️ Tạm dừng

---

## 🏗️ Kiến trúc Hệ thống

```
┌──────────────────────────────────────────────────────────────┐
│                    BEACON ECOSYSTEM                          │
│                                                              │
│  ┌─────────────┐    ┌──────────────┐    ┌────────────────┐  │
│  │ beacon_app  │    │ beacon_core  │    │ beacon_server  │  │
│  │  (Flutter)  │◄──►│   (Rust)     │    │   (Rust/Axum)  │  │
│  │             │    │              │    │                │  │
│  │ • UI/UX     │    │ • GPS        │    │ • REST API     │  │
│  │ • Screens   │    │ • BLE        │    │ • WebSocket    │  │
│  │ • Widgets   │    │ • Crypto     │    │ • Auth (JWT)   │  │
│  │ • Services  │    │ • Audio      │    │ • Twilio       │  │
│  └─────────────┘    │ • Network    │    │ • FCM          │  │
│                     │ • SMS        │    │                │  │
│                     └──────────────┘    └───────┬────────┘  │
│                                                 │           │
│  ┌──────────────┐                     ┌────────┴────────┐  │
│  │beacon_shared │                     │   Databases     │  │
│  │   (Rust)     │                     │ PostgreSQL 16   │  │
│  │ Common types │                     │ Redis 7 (Geo)   │  │
│  └──────────────┘                     └─────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

---

## 🔧 Công nghệ & Dependencies

### Rust Crates cần thiết

| Crate | Mục đích | Version | Đã cài? |
|-------|----------|---------|---------|
| `axum` | Web framework | 0.7 | ✅ workspace |
| `tokio` | Async runtime | 1.37 | ✅ workspace |
| `sqlx` | PostgreSQL driver (async) | 0.8 | ✅ workspace + beacon_shared |
| `redis` | Redis client | 0.25 | ✅ workspace |
| `serde` / `serde_json` | Serialization | 1.0 | ✅ workspace + beacon_shared |
| `jsonwebtoken` | JWT auth | 9.3 | ✅ workspace |
| `tower-http` | HTTP middleware (CORS, logging, rate-limit) | latest | 🔲 |
| `argon2` | Password/PIN hashing | 0.5 | ✅ workspace |
| `aes-gcm` | AES-256-GCM encryption | latest | 🔲 |
| `reqwest` | HTTP client (Twilio API) | 0.12 | ✅ workspace |
| `tracing` + `tracing-subscriber` | Logging | 0.1 / 0.3 | ✅ workspace |
| `uuid` | Unique IDs | 1.8 | ✅ workspace + beacon_shared |
| `chrono` | Date/time | 0.4 | ✅ workspace + beacon_shared |
| `dotenvy` | .env config | 0.15 | ✅ workspace |
| `thiserror` / `anyhow` | Error handling | 1.0 | ✅ workspace |
| `validator` | Input validation | latest | 🔲 |
| `flutter_rust_bridge` | Flutter ↔ Rust FFI | 2.0.0-dev.32 | ✅ workspace (chưa dùng) |

### Flutter Packages cần thiết

| Package | Mục đích | Đã cài? |
|---------|----------|---------|
| `flutter_rust_bridge` | Rust integration | 🔲 |
| `geolocator` | GPS location | 🔲 |
| `firebase_messaging` | Push notifications | 🔲 |
| `flutter_local_notifications` | Local notifications | 🔲 |
| `dio` | HTTP client | 🔲 |
| `flutter_secure_storage` | Secure token storage | 🔲 |
| `provider` / `riverpod` | State management | 🔲 |
| `go_router` | Navigation | 🔲 |
| `home_widget` | Lock screen widget | 🔲 |
| `flutter_blue_plus` | BLE (backup nếu Rust BLE phức tạp) | 🔲 |
| `permission_handler` | System permissions | 🔲 |
| `vibration` | Haptic feedback | 🔲 |
| `pin_code_fields` | PIN input UI | 🔲 |
| `google_maps_flutter` | Map hiển thị vị trí | 🔲 |

### Dịch vụ Bên ngoài

| Dịch vụ | Mục đích | Tài khoản? | API Key? |
|---------|----------|------------|----------|
| Twilio | SMS + Voice call | 🔲 Chưa có | 🔲 |
| Firebase | FCM Push Notifications | 🔲 Chưa có | 🔲 |
| Google Maps API | Bản đồ hiển thị vị trí | 🔲 Chưa có | 🔲 |
| S3/Backblaze B2 | Object storage (recordings) | 🔲 Chưa có | 🔲 |

### Môi trường Phát triển

| Tool | Yêu cầu | Đã cài? |
|------|---------|---------|
| Rust (rustup) | stable + nightly | ✅ Đã cài (cargo check OK) |
| Flutter SDK | 3.x | ✅ Đã cài (beacon_app build OK) |
| Docker & Docker Compose | PostgreSQL + Redis | ✅ Đã cài & chạy |
| Android Studio / SDK | Android build | ⬜ Kiểm tra |
| Xcode (macOS only) | iOS build | ⬜ N/A (Windows) |
| PostgreSQL 16 | Database | ✅ Docker container `beacon_db` |
| Redis 7 | Geospatial cache | ✅ Docker container `beacon_redis` |
| sqlx-cli | DB migrations | 🔲 Chưa cài |

---

## 📋 Chi tiết Công việc từng Phase

### PHASE 0: Foundation & Setup (Tuần 1-2)

- [ ] **Cấu trúc dự án**
  - [x] Khởi tạo Rust workspace `Cargo.toml` root
  - [x] Tạo crate `beacon_shared` (shared types)
  - [x] Tạo crate `beacon_server` (Axum backend)
  - [x] Tạo crate `beacon_core` (mobile Rust logic)
  - [x] Khởi tạo Flutter project `beacon_app` (Đã dùng đường dẫn tuyệt đối)
  - [ ] Setup `flutter_rust_bridge` (Flutter ↔ Rust)

- [ ] **Infrastructure**
  - [x] Tạo `docker-compose.yml` (PostgreSQL 16 + Redis 7)
  - [x] Tạo `.env.example` với tất cả config keys
  - [x] Viết database migrations (SQLx)
  - [x] Tạo `Makefile` / scripts tiện ích

- [ ] **CI/CD**
  - [ ] GitHub Actions: Rust build + test
  - [ ] GitHub Actions: Flutter build + test
  - [ ] Pre-commit hooks (cargo fmt, clippy)

---

### PHASE 1: Authentication & Core SOS (Tháng 1-2)

#### 1.1 Backend - Auth System
- [x] Model `User` + migration
- [x] `POST /api/v1/auth/register` → Gửi OTP qua Twilio
- [ ] `POST /api/v1/auth/verify-otp` → Xác thực OTP, tạo JWT
- [ ] `POST /api/v1/auth/refresh` → Refresh access token
- [ ] JWT middleware (tower layer)
- [ ] Rate limiting middleware

#### 1.2 Backend - User & Contacts
- [ ] Model `EmergencyContact` + migration (✅ Model done)
- [ ] `GET /api/v1/user/profile`
- [ ] `PUT /api/v1/user/profile`
- [ ] `PUT /api/v1/user/emergency-contacts`
- [ ] `PUT /api/v1/user/settings`
- [ ] Input validation (validator crate)

#### 1.3 Backend - Core SOS Flow
- [ ] Model `SosEvent` + `SosEventLog` + migrations (✅ SosEvent model done)
- [ ] `POST /api/v1/sos/activate` → Tạo event + trigger Twilio
- [ ] `POST /api/v1/sos/cancel` → Verify PIN + hủy event
- [ ] `POST /api/v1/sos/location` → Nhận GPS updates
- [ ] Twilio Service: SMS gửi link Google Maps
- [ ] Twilio Service: Voice call tới 5 contacts
- [ ] Background job: Retry failed notifications

#### 1.4 Flutter App - MVP UI
- [ ] Splash screen + Onboarding flow
- [ ] Login screen (nhập SĐT)
- [ ] OTP verification screen
- [ ] Home screen: Nút SOS lớn (animated, gradient)
- [ ] SOS countdown screen (5 giây đếm ngược + haptic)
- [ ] SOS active screen (hiển thị trạng thái, nút Cancel)
- [ ] Cancel dialog (nhập PIN)
- [ ] Emergency contacts management screen
- [ ] Settings screen (PIN, keywords, preferences)
- [ ] Profile screen

#### 1.5 Rust Core - Mobile
- [ ] GPS module: Permission request + tracking
- [ ] HTTP client: Gửi location updates lên server
- [ ] Offline queue: SQLite buffer khi mất mạng
- [ ] Auto-sync khi có lại kết nối

---

### PHASE 2: Community Radar & Battery Optimization (Tháng 3-5)

#### 2.1 Backend - Geospatial Radar
- [ ] Redis GEOADD integration
- [ ] `POST /api/v1/radar/update-location` (vị trí ẩn danh)
- [ ] `GET /api/v1/radar/nearby` (GEOSEARCH bán kính 500m-1km)
- [ ] WebSocket server (`/ws/v1/events`) cho realtime broadcast
- [ ] SOS event broadcast tới nearby users

#### 2.2 Push Notification System
- [ ] Firebase Admin SDK integration
- [ ] FCM token registration endpoint
- [ ] High-priority SOS notification (vượt silent mode)
- [ ] iOS Critical Alert entitlement
- [ ] Notification payload: tọa độ + khoảng cách + hướng
- [ ] Fallback: Twilio SMS nếu push fail

#### 2.3 Rust Core - Battery Optimization
- [ ] Accelerometer-based motion detection
- [ ] Adaptive GPS polling (3 phút → 10 phút khi đứng yên)
- [ ] Battery level monitoring
- [ ] Benchmark: Target < 2% pin/ngày chế độ chờ

#### 2.4 Lock Screen Widget
- [ ] Android: App Widget + Foreground Service
- [ ] iOS: WidgetKit + Live Activity cho SOS status
- [ ] Quick SOS trigger từ widget

#### 2.5 Flutter App - Radar UI
- [ ] Map screen hiển thị nearby users (ẩn danh)
- [ ] SOS alert popup cho nearby users
- [ ] Navigation tới vị trí nạn nhân
- [ ] Notification history screen

---

### PHASE 3: BLE, Blackbox & Advanced (Tháng 6-7)

#### 3.1 Rust Core - Bluetooth LE
- [ ] BLE scanner module
- [ ] Connection manager (auto-reconnect)
- [ ] Button press detection (iTag, Nut Find...)
- [ ] Battery monitoring thiết bị BLE
- [ ] Flutter UI: BLE device pairing screen

#### 3.2 Blackbox Recording
- [ ] Audio capture (Android MediaRecorder / iOS AVAudioEngine)
- [ ] AES-256-GCM encryption trong Rust Core
- [ ] Chunked upload (30s segments)
- [ ] `POST /api/v1/sos/recording` endpoint
- [ ] Server: S3-compatible object storage
- [ ] Flutter UI: Recording indicator

#### 3.3 Fake Call
- [ ] `POST /api/v1/fakecall/trigger` endpoint
- [ ] Twilio: Server gọi lại user
- [ ] Flutter UI: Fake incoming call screen (realistic)
- [ ] Quick trigger widget
- [ ] Customizable caller name & ringtone

#### 3.4 Voice Activation
- [ ] Android: Custom Accessibility Service
- [ ] Speech Recognition integration
- [ ] iOS: Siri Shortcuts
- [ ] Custom wake word configuration
- [ ] Flutter UI: Voice keyword setup screen

---

### PHASE 4: Testing, Security & Release (Tháng 8)

#### 4.1 Testing
- [ ] Unit tests: Rust backend (cargo test)
- [ ] Unit tests: Flutter (flutter test)
- [ ] Integration tests: API endpoints
- [ ] Load test: k6 scripts (100k concurrent SOS)
- [ ] E2E test: Full SOS flow trên thiết bị thật
- [ ] Battery consumption test (24h background)

#### 4.2 Security
- [ ] OWASP Top 10 checklist review
- [ ] Penetration testing (API + mobile)
- [ ] Data encryption at rest verification
- [ ] SSL/TLS certificate setup
- [ ] Rate limiting stress test
- [ ] Location data anonymization audit

#### 4.3 Legal & Compliance
- [ ] Privacy Policy (Tiếng Việt + English)
- [ ] Terms of Service
- [ ] Nghị định 13/2023/NĐ-CP compliance check
- [ ] Consent form cho ghi âm
- [ ] Data deletion mechanism

#### 4.4 Store Submission
- [ ] Google Play: Background location justification
- [ ] Google Play: Screenshots + description + ASO
- [ ] Apple App Store: Critical alert entitlement
- [ ] Apple App Store: Screenshots + description
- [ ] App review preparation document

#### 4.5 Open Source
- [ ] Tách beacon_core → public repo
- [ ] README.md + Contributing guide
- [ ] Code of Conduct
- [ ] LICENSE (MIT hoặc Apache 2.0)
- [ ] Documentation site (mdBook)

---

## 🗄️ Database Schema

### PostgreSQL Tables

```sql
-- Đã tạo migration? 🔲

-- users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone VARCHAR(20) UNIQUE NOT NULL,
    name VARCHAR(100),
    avatar_url TEXT,
    role VARCHAR(20) DEFAULT 'user',  -- user, responder, admin
    pin_hash VARCHAR(255),
    voice_keyword VARCHAR(100),
    is_verified BOOLEAN DEFAULT FALSE,
    trust_score INTEGER DEFAULT 30,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- device_fingerprints
CREATE TABLE device_fingerprints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    device_hash VARCHAR(255) NOT NULL,
    platform VARCHAR(10),
    is_emulator BOOLEAN DEFAULT FALSE,
    first_seen TIMESTAMPTZ DEFAULT NOW(),
    last_seen TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, device_hash)
);

-- emergency_contacts
CREATE TABLE emergency_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    phone VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    relationship VARCHAR(50),
    priority SMALLINT NOT NULL,  -- 1-5
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- sos_events
CREATE TABLE sos_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    status VARCHAR(20) DEFAULT 'active',  -- active, cancelled, resolved
    trigger_method VARCHAR(20),  -- button, ble, voice, widget
    created_at TIMESTAMPTZ DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

-- sos_event_logs (location history)
CREATE TABLE sos_event_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID REFERENCES sos_events(id) ON DELETE CASCADE,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    battery_level SMALLINT,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);

-- recordings
CREATE TABLE recordings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID REFERENCES sos_events(id) ON DELETE CASCADE,
    file_url TEXT NOT NULL,
    encryption_key_hash VARCHAR(255),
    duration_seconds INTEGER,
    file_size_bytes BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- devices (FCM tokens + BLE devices)
CREATE TABLE devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    fcm_token TEXT,
    platform VARCHAR(10),  -- android, ios
    ble_device_id VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- notifications_log
CREATE TABLE notifications_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID REFERENCES sos_events(id),
    recipient_user_id UUID REFERENCES users(id),
    type VARCHAR(20),  -- sms, voice, push, nearby
    status VARCHAR(20),  -- sent, delivered, failed
    sent_at TIMESTAMPTZ DEFAULT NOW()
);

-- otp_codes
CREATE TABLE otp_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone VARCHAR(20) NOT NULL,
    code VARCHAR(6) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Redis Keys

```
geo:active_users              → GEOADD (lng, lat, user_id)
user:{id}:location            → HASH {lat, lng, updated_at}
user:{id}:session             → STRING (refresh_token_hash) TTL 30d
sos:active                    → SET {event_id, event_id, ...}
sos:event:{id}                → HASH {user_id, lat, lng, status}
rate_limit:otp:{phone}        → STRING with TTL 60s
rate_limit:sos:{user_id}      → STRING with TTL (max 3/day)
```

---

## 💰 Chi phí Ước tính

| Hạng mục | /tháng | Ghi chú |
|----------|--------|---------|
| VPS (2 vCPU, 4GB RAM) | ~$20-40 | DigitalOcean/Vultr |
| Redis | ~$5-15 | Hoặc self-hosted |
| PostgreSQL | $0 | Self-hosted trên VPS |
| Twilio SMS | ~$0.05/SMS | Pay per use |
| Twilio Voice | ~$0.015/phút | Cuộc gọi ngắn |
| Firebase FCM | Miễn phí | Unlimited notifications |
| Object Storage | ~$5-20 | Recordings |
| Domain + SSL | ~$1-2 | Let's Encrypt free SSL |
| **Tổng (giai đoạn đầu)** | **~$30-80** | Scale theo users |

---

## 📝 Nhật ký Phát triển (Development Log)

> Ghi lại mỗi session làm việc: ngày, việc đã làm, vấn đề gặp phải, việc cần làm tiếp.

### 2026-06-21 (Pair Programming Session)
- **Việc đã làm:**
  - ✅ Cấu trúc lại `beacon_server`: Tách rời `config.rs`, `app_state.rs` và `routes/mod.rs` khỏi `main.rs`.
  - ✅ Thiết lập **Global Error Handler** (`errors.rs`) sử dụng enum `AppError` và implement trait `IntoResponse`, tích hợp log.
- **Vấn đề:** Đã giải quyết bug throw thiếu response body khi handle lỗi và tích hợp `From<sqlx::Error>`.
- **Việc cần làm tiếp:**
  - ✅ Xây dựng API Đăng ký `POST /api/v1/auth/register` (nhận SĐT, tạo user, gửi OTP).

### 2026-06-18
- **Việc đã làm (Buổi sáng):**
  - ✅ Kiểm tra và đồng bộ hóa file migration `init.sql` với tài liệu thiết kế. Đã bổ sung 4 bảng còn thiếu.
  - ✅ Khởi tạo lại Database (PostgreSQL) thành công với tổng cộng 9 bảng, giải quyết triệt để lỗi bất đồng bộ.
  - ✅ Thêm trường `trust_score` vào model `User` trong Rust và Schema của Tracker.
  - ✅ Xuất code DBML (DBDiagram) để biểu diễn trực quan hóa kiến trúc Database.
  - ✅ Setup nền móng Backend: Axum Web Server + SQLx Connection Pool trong `beacon_server/src/main.rs`.
  - ✅ Khai báo đầy đủ dependencies cho `beacon_server` (axum, tokio, sqlx, tracing, dotenvy).
  - ✅ Tạo `AppState` struct chứa `PgPool` và inject vào Router qua `.with_state()`.
  - ✅ Tạo endpoint `GET /health` (Health Check) cho hệ thống.
  - ✅ Cấu hình đọc `PORT` và `DATABASE_URL` từ file `.env` (không hardcode).
  - ✅ Code Review & Refactor: Sửa lỗi naming convention (snake_case), thay `unwrap()` bằng `expect()`, bổ sung logging (`tracing::info!`).
  - ✅ `cargo check` — Biên dịch thành công, 0 warning, 0 error.
- **Việc cần làm tiếp (Buổi chiều):**
  - 🔜 Backend: Tách cấu trúc thư mục `beacon_server` (routes/, handlers/, config/) để chuẩn bị scale.
  - ✅ Backend: Viết API `POST /api/v1/auth/register` (Đăng ký bằng SĐT + gửi OTP).
  - 🔜 Backend: JWT middleware (tower layer) cho xác thực.
  - 🔜 Mobile: Setup `flutter_rust_bridge` để kết nối Flutter và Rust.

### 2026-06-17
- **Việc đã làm:**
  - ✅ Phân tích tài liệu PDF kế hoạch dự án (5 trang)
  - ✅ Đánh giá tính khả thi → KHẢ THI
  - ✅ Nhận diện 9 phần thiếu sót và bổ sung đầy đủ
  - ✅ Tạo Implementation Plan chi tiết
  - ✅ Tạo Project Tracker (file này)
  - ✅ Bắt đầu Phase 1: Tạo Model `User` trong `beacon_shared`
  - ✅ Hoàn thành Phase 0: Khởi tạo project Flutter `beacon_app`
  - ✅ Thử nghiệm biên dịch và khởi chạy thành công `beacon_app` trên Windows Desktop
  - ✅ Khởi động thành công Docker (PostgreSQL 16, Redis 7) qua `docker-compose`.
  - ✅ Hoàn thiện các Model `EmergencyContact` và `SosEvent` trong `beacon_shared`.
- **Vấn đề:**
  - Chưa xác nhận: Tên dự án chính thức (Beacon vs SOS Rescue Ecosystem)
  - Chưa xác nhận: flutter_rust_bridge vs UniFFI
  - Chưa xác nhận: Kinh nghiệm Flutter của developer
  - Chưa xác nhận: Có Twilio/Firebase account chưa
  - Chưa xác nhận: Target Android only hay cả iOS
  - ✅ **Đã giải quyết**: Docker Engine đã chạy thành công cùng PostgreSQL và Redis.
- **Việc cần làm tiếp:**
  - 🔜 Backend: Setup Axum server (`beacon_server`), cấu hình kết nối Database (SQLx).
  - 🔜 Backend: Viết database migrations cho các bảng đã định nghĩa.
  - ✅ Backend: Viết API `POST /api/v1/auth/register`.
  - 🔜 Mobile: Setup `flutter_rust_bridge` để kết nối Flutter và Rust.

---

## ⚡ Quyết định Kỹ thuật (Technical Decisions)

> Ghi lại các quyết định quan trọng trong quá trình phát triển.

| # | Quyết định | Lựa chọn | Lý do | Ngày |
|---|-----------|----------|-------|------|
| 1 | Backend framework | Axum | tower ecosystem, tokio team maintain | 2026-06-17 |
| 2 | Database | PostgreSQL 16 + Redis 7 | Geo queries + relational data | 2026-06-17 |
| 3 | Auth method | JWT + OTP SMS | Phù hợp target user VN | 2026-06-17 |
| 4 | Encryption | AES-256-GCM | Industry standard E2E | 2026-06-17 |
| 5 | Flutter ↔ Rust bridge | TBD | flutter_rust_bridge đề xuất | - |
| 6 | State management (Flutter) | TBD | Riverpod đề xuất | - |

---

## 🛡️ Hệ thống Chống Lạm dụng & Lừa đảo (Anti-Abuse System)

> **VẤN ĐỀ CỐT LÕI:** Kẻ xấu có thể lợi dụng SOS giả để DỤ DỖ người cứu hộ đến vị trí nguy hiểm,
> spam gây mất niềm tin cộng đồng, hoặc lợi dụng radar để theo dõi người khác.

### Kịch bản Lạm dụng & Mức độ Nguy hiểm

| # | Kịch bản | Mức độ | Mô tả |
|---|----------|:---:|---|
| 1 | 🔴 **Dụ dỗ/bẫy người cứu hộ** | Cực kỳ nguy hiểm | Phát SOS giả ở nơi vắng → dụ người đến → cướp/tấn công |
| 2 | 🟠 **Spam SOS giả** | Nghiêm trọng | Tạo hàng loạt SOS giả → cộng đồng mất tin tưởng → bỏ qua SOS thật |
| 3 | 🟠 **Stalking/theo dõi** | Nghiêm trọng | Lợi dụng radar xác định vị trí nạn nhân rồi theo dõi |
| 4 | 🟡 **Tài khoản ảo** | Trung bình | Tạo nhiều tài khoản fake để spam hoặc bypass ban |
| 5 | 🟡 **Mạo danh cứu hộ** | Trung bình | Giả danh tổ chức cứu hộ để tiếp cận nạn nhân với ý đồ xấu |

### TẦNG 1: Xác minh Danh tính Chặt chẽ (Identity Verification)

- [ ] **Xác minh SĐT bắt buộc** qua OTP — 1 SĐT = 1 tài khoản duy nhất
- [ ] **Device fingerprinting** — Giới hạn 2 thiết bị/tài khoản, phát hiện máy ảo/emulator
- [ ] **CCCD/CMND verification (Phase nâng cao)** — Xác minh danh tính thật cho tài khoản "Đáng tin cậy"
- [ ] **Chặn SĐT ảo** — Blacklist các dải số VoIP, virtual numbers (TextNow, Google Voice...)

```sql
-- Database: Bảng theo dõi thiết bị
CREATE TABLE device_fingerprints (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    device_hash VARCHAR(255) NOT NULL,  -- Hash của hardware ID
    platform VARCHAR(10),
    is_emulator BOOLEAN DEFAULT FALSE,
    first_seen TIMESTAMPTZ DEFAULT NOW(),
    last_seen TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, device_hash)
);
```

### TẦNG 2: Hệ thống Điểm Tin cậy (Trust Score System) ⭐ QUAN TRỌNG

> Mỗi user có một **Trust Score (0-100)** quyết định quyền hạn trong hệ thống.

- [ ] **Tài khoản mới**: Trust Score = 30 (giới hạn tính năng)
- [ ] **Tăng điểm khi**: Xác minh SĐT (+10), Thêm avatar (+5), Sử dụng app > 7 ngày (+10), Phản hồi SOS người khác (+15), Xác minh CCCD (+20)
- [ ] **Giảm điểm khi**: SOS bị báo cáo fake (-30), Hủy SOS quá nhiều (-10/lần), Bị report bởi người khác (-20)
- [ ] **Trust Score < 20**: Tài khoản bị **đình chỉ SOS** (chỉ nhận cảnh báo, không phát được)
- [ ] **Trust Score < 10**: Tài khoản bị **ban vĩnh viễn** + SĐT vào blacklist

```rust
// Rust: Trust Score calculation
pub struct TrustScore {
    pub score: i32,           // 0-100
    pub total_sos: u32,       // Tổng số SOS đã phát
    pub false_sos_count: u32, // Số lần bị báo cáo fake
    pub rescue_responses: u32, // Số lần đi cứu người khác
    pub account_age_days: u32,
    pub is_verified: bool,    // Đã xác minh CCCD
}

impl TrustScore {
    pub fn can_activate_sos(&self) -> bool {
        self.score >= 20
    }
    
    pub fn sos_reach_radius(&self) -> u32 {
        match self.score {
            0..=29  => 0,     // Không được phát SOS radar
            30..=49 => 200,   // Bán kính 200m (hạn chế)
            50..=79 => 500,   // Bán kính 500m (tiêu chuẩn)
            80..=100 => 1000, // Bán kính 1km (đáng tin cậy)
            _ => 0,
        }
    }
}
```

### TẦNG 3: Rate Limiting & Phát hiện Bất thường

- [ ] **Giới hạn SOS**: Tối đa 3 lần SOS/ngày/tài khoản
- [ ] **Cooldown**: Sau khi hủy SOS → phải chờ 30 phút mới phát lại
- [ ] **Pattern detection**: Phát hiện mẫu bất thường:
  - SOS liên tục từ cùng vị trí nhưng luôn hủy trong < 60 giây
  - SOS ở vị trí vắng vẻ (low population density) + tài khoản mới
  - SOS ban đêm (23h-5h) từ tài khoản Trust Score thấp → gắn cờ cảnh báo
- [ ] **Exponential backoff**: SOS thứ 2 trong ngày cần xác nhận thêm (nhập PIN + countdown dài hơn 10s)

```
Redis Rate Limiting:
  rate_limit:sos:{user_id}:daily    → INCR, EXPIRE 86400  (max 3)
  rate_limit:sos:{user_id}:cooldown → SET, EXPIRE 1800    (30 phút)
  anomaly:sos:{user_id}:cancel_fast → INCR, EXPIRE 86400  (đếm hủy nhanh)
```

### TẦNG 4: Bảo vệ Người Cứu hộ (Rescuer Protection) ⭐ QUAN TRỌNG NHẤT

> **Kịch bản nguy hiểm nhất**: Kẻ xấu phát SOS giả để dụ người đến nơi vắng.

- [ ] **Hiển thị Trust Score của người gọi SOS** trên thông báo cứu hộ:
  - 🟢 `Trust Score 80+`: "Tài khoản đáng tin cậy ✓"
  - 🟡 `Trust Score 30-79`: "Tài khoản thông thường"
  - 🔴 `Trust Score < 30`: "⚠️ Tài khoản mới — hãy cẩn trọng"
- [ ] **Cảnh báo vùng nguy hiểm**: Nếu SOS ở khu vực vắng (dựa trên mật độ user), hiển thị cảnh báo "Khu vực ít người — nên đi cùng người khác"
- [ ] **Nút SOS cho người cứu hộ**: Khi đến hiện trường, người cứu hộ cũng có nút SOS riêng → nếu bị tấn công, lập tức báo động tiếp
- [ ] **Auto-tracking khi đi cứu**: Khi nhấn "Tôi đang đến", vị trí người cứu hộ được ghi lại → nếu mất liên lạc > 15 phút → tự động cảnh báo
- [ ] **Chia sẻ nhóm**: Khuyến khích đi cứu theo nhóm ≥ 2 người (hiển thị "3 người khác cũng đang đến")

### TẦNG 5: Chống Theo dõi (Anti-Stalking)

- [ ] **Vị trí ẩn danh**: Radar chỉ hiện khoảng cách tương đối (120m) + hướng, KHÔNG hiện vị trí chính xác
- [ ] **Jittering**: Thêm nhiễu ±50m vào tọa độ hiển thị trên radar (trừ khi SOS active)
- [ ] **Phantom users**: Server tạo thêm "user ảo" trên radar → kẻ xấu không biết đâu là thật
- [ ] **Chế độ ẩn**: User có thể tắt radar visibility (không xuất hiện trên bản đồ người khác)
- [ ] **Cooldown xem vị trí**: Không cho phép query vị trí 1 người cụ thể liên tục

#### 🔴 Phòng chống Location Inference Attack (Tam giác hóa vị trí)

> ⚠️ **LỖ HỔNG ĐÃ PHÁT HIỆN**: Kẻ xấu tạo nhiều tài khoản, đứng ở nhiều vị trí khác nhau,
> liên tục gọi `GET /radar/nearby` → thu thập khoảng cách tới target → vẽ vòng tròn giao nhau
> → **suy luận ra vị trí chính xác** của người dùng khác (Trilateration Attack).
> Đây là dạng tấn công phổ biến ở mọi ứng dụng xã hội dựa trên GPS.

```
Kịch bản tấn công:
  Attacker tạo 5 tài khoản (A1..A5), mỗi tài khoản ở 1 vị trí khác nhau

  A1 (10.762, 106.660) → thấy nạn nhân cách 1.2km
  A2 (10.770, 106.665) → thấy nạn nhân cách 0.8km
  A3 (10.758, 106.670) → thấy nạn nhân cách 1.5km

  → Vẽ 3 vòng tròn (Trilateration) → giao điểm = vị trí chính xác nạn nhân
```

##### Lớp phòng thủ 5.1: Spatial Cloaking (Ưu tiên P0)

> **Không bao giờ trả về khoảng cách chính xác** — chỉ trả về bucket thô.

- [ ] Response từ `/radar/nearby` chỉ trả khoảng cách theo bậc thang: `< 500m`, `< 1km`, `< 2km`, `< 5km`
- [ ] KHÔNG trả về giá trị khoảng cách số thực (vd: 847.32m)

```rust
// ❌ SAI — trả về khoảng cách chính xác → attacker tam giác hóa được
{ "distance_m": 847.32 }

// ✅ ĐÚNG — chỉ trả bucket thô → tam giác hóa thất bại
{ "distance_label": "< 1km" }
// Các bậc: < 500m, < 1km, < 2km, < 5km
```

##### Lớp phòng thủ 5.2: Grid Snapping — Làm mờ tọa độ (Ưu tiên P0)

> Snap vị trí người dùng vào **lưới ô vuông ~200m** trước khi tính toán.
> Mọi người trong cùng ô vuông cho kết quả giống hệt nhau → attacker không phân biệt được.

- [ ] Implement hàm `snap_to_grid()` trên server trước khi xử lý bất kỳ query radar nào

```rust
fn snap_to_grid(lat: f64, lon: f64) -> (f64, f64) {
    let precision = 0.002; // ~200m
    let snapped_lat = (lat / precision).round() * precision;
    let snapped_lon = (lon / precision).round() * precision;
    (snapped_lat, snapped_lon)
}
```

##### Lớp phòng thủ 5.3: Rate Limiting theo vị trí (Ưu tiên P1)

- [ ] Giới hạn mỗi user: tối đa **5 lần query radar / phút**, **20 lần / giờ**
- [ ] Nếu vị trí query thay đổi > 3 lần trong 5 phút → gắn cờ `SUSPICIOUS_MOVEMENT`
- [ ] Redis key: `rate_limit:radar:{user_id}:minute` TTL 60s, `rate_limit:radar:{user_id}:hour` TTL 3600s

##### Lớp phòng thủ 5.4: Random Offset Injection (Ưu tiên P1)

> Thêm **nhiễu ngẫu nhiên ±15%** vào kết quả mỗi lần query.
> Mỗi lần gọi API cùng một cặp (querier, target) → kết quả khác nhau → tam giác hóa bị sai.

- [ ] Implement noise injection vào distance trước khi trả response

```rust
use rand::Rng;

fn add_noise(distance_m: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let noise = rng.gen_range(-0.15..0.15); // ±15%
    distance_m * (1.0 + noise)
}
// 1000m → có thể trả về 850m hoặc 1150m (mỗi lần khác nhau)
```

##### Lớp phòng thủ 5.5: Result Cooldown (Ưu tiên P2)

- [ ] Cùng một cặp (querier_id, target_id), **cache kết quả 5 phút** trong Redis
- [ ] Dù attacker di chuyển và query lại → vẫn nhận kết quả cũ → tam giác hóa sai hoàn toàn
- [ ] Redis key: `radar:cache:{querier_id}:{target_id}` TTL 300s

##### Lớp phòng thủ 5.6: Anomaly Detection — Phát hiện bất thường (Ưu tiên P2)

- [ ] Background job phát hiện pattern bất thường:
  - User query radar > 50 lần/ngày → cảnh báo admin
  - User thay đổi vị trí > 10 lần/giờ, khoảng cách > 5km → nghi ngờ bot/spoof GPS
  - Nhiều user query cùng 1 target trong thời gian ngắn → nghi ngờ coordinated attack
- [ ] Kết hợp với **Device Fingerprint** (Tầng 1): nếu 5 tài khoản dùng cùng device_hash/IP range → FLAG & BLOCK

##### Ma trận phòng thủ Location Inference Attack

| Lớp | Chống được gì | Độ phức tạp | Ưu tiên |
|------|---------------|-------------|---------|
| Spatial Cloaking | Tam giác hóa chính xác | Thấp | **P0** |
| Grid Snapping | Suy luận vị trí chính xác | Thấp | **P0** |
| Rate Limiting vị trí | Brute-force query | Trung bình | **P1** |
| Random Offset | Phân tích thống kê | Thấp | **P1** |
| Result Cooldown | Query liên tục lấy data mới | Thấp | **P2** |
| Anomaly Detection | Tấn công phối hợp đa tài khoản | Cao | **P2** |

##### Ngoại lệ khi SOS Active

> **Khi KHÔNG có SOS event**: Áp dụng **tất cả 6 lớp** — bảo vệ privacy tối đa.
>
> **Khi CÓ SOS event đang active**: Chỉ **nới lỏng** cho rescuer đã verify (Trust Score >= 50)
> trong bán kính phù hợp. Cần cân bằng giữa privacy và tính mạng.

```rust
pub fn get_location_precision(has_active_sos: bool, rescuer_trust: i32) -> LocationPrecision {
    if has_active_sos && rescuer_trust >= 50 {
        LocationPrecision::Exact       // Ưu tiên cứu mạng → cho vị trí chính xác
    } else if has_active_sos && rescuer_trust >= 30 {
        LocationPrecision::Grid200m    // Gần chính xác, nhưng vẫn có grid
    } else {
        LocationPrecision::BucketOnly  // Bình thường → chỉ bucket thô (< 500m, < 1km...)
    }
}
```


### TẦNG 6: Hệ thống Báo cáo Chống Lạm dụng Ngược (Anti-Weaponized Report System)

> ⚠️ **LỖ HỔNG ĐÃ PHÁT HIỆN**: Nhóm kẻ xấu (5 người) có thể dàn cảnh → bật SOS giả hoặc
> tấn công nạn nhân thật → rồi đồng loạt REPORT nạn nhân → khiến nạn nhân bị BAN
> → nạn nhân mất khả năng cầu cứu. **ĐÂY LÀ THẢM HỌA.**

#### NGUYÊN TẮC SỐ 1 (BẤT KHẢ XÂM PHẠM):

```
┌─────────────────────────────────────────────────────────────────┐
│  🔒 REPORT KHÔNG BAO GIỜ ĐƯỢC DỪNG MỘT SOS ĐANG HOẠT ĐỘNG    │
│                                                                 │
│  • SMS/Call tới 5 người thân    → LUÔN LUÔN được gửi           │
│  • Radar cảnh báo cộng đồng    → LUÔN LUÔN hoạt động           │
│  • GPS tracking lên server     → LUÔN LUÔN ghi nhận            │
│  • Blackbox recording          → LUÔN LUÔN ghi âm              │
│                                                                 │
│  Report chỉ được XỬ LÝ SAU khi SOS event kết thúc + 72h.      │
└─────────────────────────────────────────────────────────────────┘
```

#### Cơ chế 1: SOS Immunity Lock (Khóa Miễn dịch khi SOS)

- [ ] Khi SOS đang `active` → tài khoản vào trạng thái **IMMUNE** (bất khả xâm phạm)
- [ ] Mọi report nhận trong thời gian SOS active → chỉ **xếp hàng chờ** (queued), KHÔNG xử lý
- [ ] Sau khi SOS kết thúc (cancelled/resolved) → chờ thêm **72 giờ** → mới review reports
- [ ] Trong 72h đó, nạn nhân vẫn sử dụng app bình thường

```rust
pub fn can_process_reports(event: &SosEvent) -> bool {
    match event.status {
        SosStatus::Active => false,  // KHÔNG BAO GIỜ xử lý khi SOS đang bật
        SosStatus::Cancelled | SosStatus::Resolved => {
            let hours_since_end = now() - event.resolved_at.unwrap();
            hours_since_end >= Duration::hours(72) // Chờ 72h sau khi kết thúc
        }
    }
}
```

#### Cơ chế 2: Report có Trọng số (Weighted Reports)

> Không phải mọi report đều có giá trị ngang nhau.

- [ ] Report từ tài khoản Trust Score < 30 → **trọng số 0.1** (gần như vô giá trị)
- [ ] Report từ tài khoản Trust Score 30-60 → **trọng số 0.5**
- [ ] Report từ tài khoản Trust Score 60-80 → **trọng số 1.0**
- [ ] Report từ tài khoản Trust Score 80+ (đã xác minh CCCD) → **trọng số 2.0**
- [ ] **Ngưỡng xử lý**: Tổng trọng số phải ≥ 5.0 mới trigger đình chỉ (chờ admin review)

```
Ví dụ kịch bản tấn công:
  5 kẻ xấu (tài khoản mới, Trust Score ~25 mỗi người)
  → 5 reports × trọng số 0.1 = 0.5 tổng
  → 0.5 < 5.0 → KHÔNG ĐỦ ĐỂ ĐÌNH CHỈ ← Tấn công THẤT BẠI

Ví dụ report hợp lệ:
  3 người dùng uy tín (Trust Score 70+, xác minh CCCD)
  → 1×2.0 + 2×1.0 = 4.0 → gần ngưỡng, admin review
```

#### Cơ chế 3: Phát hiện Thông đồng (Anti-Collusion Detection)

- [ ] **Phát hiện nhóm report**: Nếu nhiều tài khoản report cùng 1 target trong < 10 phút → gắn cờ `COLLUSION_SUSPECTED`
- [ ] **Phân tích đồ thị quan hệ (Social Graph)**:
  - Các reporter từng ở gần nhau thường xuyên? → nghi ngờ quen biết
  - Các reporter có cùng device fingerprint/IP range? → nghi ngờ cùng 1 người
  - Các reporter tạo tài khoản cùng thời điểm? → nghi ngờ dàn cảnh
- [ ] **Cluster detection**: Nếu 3+ reporter luôn report chung → đánh dấu là **Report Ring** → vô hiệu hóa toàn bộ report từ nhóm đó

```sql
-- Phát hiện collusion: Tìm nhóm users luôn report chung
SELECT reporter_id, COUNT(DISTINCT target_event_id) as shared_reports
FROM sos_reports r1
WHERE EXISTS (
    SELECT 1 FROM sos_reports r2
    WHERE r2.target_event_id = r1.target_event_id
    AND r2.reporter_id != r1.reporter_id
    AND ABS(EXTRACT(EPOCH FROM (r2.created_at - r1.created_at))) < 600 -- < 10 phút
)
GROUP BY reporter_id
HAVING COUNT(DISTINCT target_event_id) >= 3; -- Report chung ≥ 3 lần → COLLUSION
```

#### Cơ chế 4: Quyền Report có Điều kiện

- [ ] **Trust Score < 40**: KHÔNG có quyền report người khác
- [ ] **Tài khoản < 14 ngày**: KHÔNG có quyền report
- [ ] **Mỗi user chỉ report được 1 lần/event**: Chặn 1 người report nhiều lần
- [ ] **Giới hạn report**: Mỗi user chỉ report tối đa 3 event/tháng → chống spam report
- [ ] **Report phải kèm lý do**: Chọn 1 trong các lý do có sẵn (không phải text tự do) + Optional ảnh/video chứng minh

#### Cơ chế 5: Reporter cũng bị Giám sát (Reporter Accountability)

- [ ] **Nếu report bị admin xác nhận là SAI** (nạn nhân thật bị report oan):
  - Reporter bị **trừ 25 Trust Score**
  - Reporter bị **cấm report 30 ngày**
  - Nếu false report ≥ 3 lần → reporter bị **ban vĩnh viễn** (kẻ xấu bị loại ngược)
- [ ] **Nếu phát hiện collusion ring**:
  - TOÀN BỘ nhóm bị **ban vĩnh viễn** + SĐT vào blacklist
  - Log toàn bộ bằng chứng → sẵn sàng cung cấp cho cơ quan chức năng

```
Kết quả kịch bản tấn công sau khi fix:

    👿 A (Trust 25) ── report ──┐
    👿 B (Trust 25) ── report ──┤
    👿 C (Trust 25) ── report ──┼──→ 🆘 Nạn nhân (SOS ACTIVE)
    👿 D (Trust 25) ── report ──┤         │
    👿 E (Trust 25) ── report ──┘         │
                                          ▼
    ✅ SOS Immunity Lock → Reports bị QUEUED, KHÔNG xử lý
    ✅ SMS/Call tới 5 người thân → ĐÃ GỬI THÀNH CÔNG
    ✅ Radar cộng đồng → ĐANG HOẠT ĐỘNG
    ✅ Blackbox recording → ĐANG GHI ÂM
    
    ... 72h sau SOS kết thúc ...
    
    ⚠️ Admin review reports:
      → 5 reporter có Trust Score < 30 → Trọng số = 0.5 (< 5.0 ngưỡng)
      → 5 reports trong < 10 phút → COLLUSION_SUSPECTED
      → 5 reporters tạo tài khoản cùng tuần → XÁC NHẬN COLLUSION
      
    ❌ Reports bị HỦY
    ❌ 5 kẻ xấu bị BAN VĨNH VIỄN ← Phản đòn
    ✅ Nạn nhân được bảo vệ toàn diện
```

#### Cơ chế 6: Kênh Cứu hộ Song song Không thể Chặn (Unkillable Lifeline)

> Ngay cả trong kịch bản TỆ NHẤT (account bị ban bằng cách nào đó), nạn nhân vẫn có đường sống.

- [ ] **SMS Fallback trực tiếp**: Rust Core trên điện thoại tự gửi SMS trực tiếp đến 5 SĐT thân nhân mà KHÔNG qua server → server bị hack hay account bị ban vẫn gửi được
- [ ] **Offline SOS**: Nếu mất mạng hoàn toàn → gửi SMS nội tại + bật còi hú trên loa
- [ ] **Emergency bypass**: Nếu account bị đình chỉ → vẫn cho phép 1 SOS khẩn cấp cuối cùng (Emergency Override) với recording tự động → admin review sau
- [ ] **Nút cứng 113/114**: Ngoài SOS cộng đồng, luôn hiển thị nút gọi thẳng 113 (Công an) / 114 (Cứu hỏa) / 115 (Cấp cứu) → không ai có thể chặn cuộc gọi khẩn cấp hệ thống

```rust
// Rust Core (chạy trên điện thoại - KHÔNG phụ thuộc server)
pub async fn emergency_sos(contacts: &[Contact; 5], location: GpsCoord) {
    // Kênh 1: Gửi qua server (có thể bị chặn bởi ban)
    let server_result = api_client::send_sos(location).await;
    
    // Kênh 2: SMS trực tiếp từ SIM (KHÔNG QUA SERVER - KHÔNG THỂ CHẶN)
    for contact in contacts {
        sms::send_direct(
            contact.phone,
            format!("🆘 SOS! Tôi cần giúp đỡ! Vị trí: https://maps.google.com/?q={},{}", 
                    location.lat, location.lng)
        ).await;
    }
    
    // Kênh 3: Nếu cả 2 kênh trên fail → bật còi hú max volume
    if server_result.is_err() {
        audio::play_siren_max_volume().await;
    }
}
```

### TẦNG 7: Răn đe Pháp lý (Legal Deterrent)

- [ ] **Cảnh báo trước khi phát SOS**: Popup "Phát SOS giả là hành vi vi phạm pháp luật. Mọi hoạt động đều được ghi nhận."
- [ ] **Cảnh báo trước khi report**: Popup "Báo cáo sai sự thật cũng là hành vi vi phạm. Tài khoản của bạn sẽ bị xử lý nếu report không trung thực."
- [ ] **Terms of Service**: Ghi rõ hậu quả pháp lý của cả hai hành vi: SOS giả VÀ report giả
- [ ] **Hợp tác cơ quan chức năng**: Cung cấp log cho công an khi có yêu cầu hợp pháp
- [ ] **Điều khoản Việt Nam**: BLHS 2015 Điều 318 (Gây rối trật tự - tù đến 7 năm) + Điều 155 (Vu khống - tù đến 3 năm)

### Tóm tắt: Ma trận Phòng thủ (Đã vá lỗ hổng Report Abuse)

```
                    ┌─────────────────────────────┐
                    │   Kẻ xấu muốn lạm dụng     │
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  TẦNG 1: Xác minh SĐT/CCCD  │ ← Chặn tài khoản ảo
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  TẦNG 2: Trust Score 0-100   │ ← Giới hạn quyền theo uy tín
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  TẦNG 3: Rate Limit + AI     │ ← Phát hiện pattern bất thường
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  TẦNG 4: Bảo vệ người cứu   │ ← Cảnh báo + tracking + nhóm
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  TẦNG 5: Anti-stalking       │ ← Ẩn danh + jitter + phantom
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  TẦNG 6: Anti-Report Abuse   │ ← SOS Immunity + Weighted Report
                    │  + Collusion Detection       │   + Phản đòn kẻ report giả
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  TẦNG 7: Pháp lý răn đe     │ ← BLHS Điều 318 + Điều 155
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │  KÊNH SONG SONG: SMS trực    │ ← KHÔNG THỂ CHẶN bằng bất kỳ
                    │  tiếp + 113/114/115          │   cách nào (bypass mọi hệ thống)
                    └─────────────────────────────┘
```

---

## 🐛 Known Issues & Blockers

> Ghi lại các vấn đề đang chặn tiến độ.

| # | Vấn đề | Mức độ | Trạng thái | Giải pháp |
|---|--------|--------|------------|-----------|
| 1 | Thiếu Flutter SDK (lệnh `flutter` not found) | High | 🔴 Blocked | Cần cài đặt Flutter SDK trên máy Windows và thêm vào PATH |
| 2 | Docker Engine chưa bật | High | 🔴 Blocked | Cần khởi động ứng dụng Docker Desktop để chạy `docker-compose up` |
| 3 | Thiếu `sqlx-cli` | Medium | 🟡 Pending | Cài bằng `cargo install sqlx-cli` sau khi database sẵn sàng |

---

## 📚 Tài liệu Tham khảo

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge/)
- [Redis Geospatial](https://redis.io/docs/data-types/geospatial/)
- [Twilio SMS API](https://www.twilio.com/docs/sms)
- [Firebase Cloud Messaging](https://firebase.google.com/docs/cloud-messaging)
- [Nghị định 13/2023/NĐ-CP](https://thuvienphapluat.vn/van-ban/Cong-nghe-thong-tin/Nghi-dinh-13-2023-ND-CP-bao-ve-du-lieu-ca-nhan-357384.aspx)
