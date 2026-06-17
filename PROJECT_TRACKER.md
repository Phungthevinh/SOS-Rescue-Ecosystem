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
| Phase 0 | Foundation & Setup | 🔲 Chưa bắt đầu | Tuần 1-2 |
| Phase 1 | Auth & Core SOS (MVP) | 🔲 Chưa bắt đầu | Tháng 1-2 |
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
| `axum` | Web framework | latest | 🔲 |
| `tokio` | Async runtime | 1.x | 🔲 |
| `sqlx` | PostgreSQL driver (async) | 0.8.x | 🔲 |
| `redis` | Redis client | latest | 🔲 |
| `serde` / `serde_json` | Serialization | 1.x | 🔲 |
| `jsonwebtoken` | JWT auth | latest | 🔲 |
| `tower-http` | HTTP middleware (CORS, logging, rate-limit) | latest | 🔲 |
| `argon2` | Password/PIN hashing | latest | 🔲 |
| `aes-gcm` | AES-256-GCM encryption | latest | 🔲 |
| `reqwest` | HTTP client (Twilio API) | latest | 🔲 |
| `tracing` + `tracing-subscriber` | Logging | latest | 🔲 |
| `uuid` | Unique IDs | 1.x | 🔲 |
| `chrono` | Date/time | latest | 🔲 |
| `dotenvy` | .env config | latest | 🔲 |
| `thiserror` / `anyhow` | Error handling | latest | 🔲 |
| `validator` | Input validation | latest | 🔲 |
| `flutter_rust_bridge` | Flutter ↔ Rust FFI | latest | 🔲 |

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
| Rust (rustup) | stable + nightly | ⬜ Kiểm tra |
| Flutter SDK | 3.x | ⬜ Kiểm tra |
| Docker & Docker Compose | PostgreSQL + Redis | ⬜ Kiểm tra |
| Android Studio / SDK | Android build | ⬜ Kiểm tra |
| Xcode (macOS only) | iOS build | ⬜ Kiểm tra |
| PostgreSQL 16 | Database | 🔲 |
| Redis 7 | Geospatial cache | 🔲 |
| sqlx-cli | DB migrations | 🔲 |

---

## 📋 Chi tiết Công việc từng Phase

### PHASE 0: Foundation & Setup (Tuần 1-2)

- [ ] **Cấu trúc dự án**
  - [ ] Khởi tạo Rust workspace `Cargo.toml` root
  - [ ] Tạo crate `beacon_shared` (shared types)
  - [ ] Tạo crate `beacon_server` (Axum backend)
  - [ ] Tạo crate `beacon_core` (mobile Rust logic)
  - [ ] Khởi tạo Flutter project `beacon_app`
  - [ ] Setup `flutter_rust_bridge` (Flutter ↔ Rust)

- [ ] **Infrastructure**
  - [ ] Tạo `docker-compose.yml` (PostgreSQL 16 + Redis 7)
  - [ ] Tạo `.env.example` với tất cả config keys
  - [ ] Viết database migrations (SQLx)
  - [ ] Tạo `Makefile` / scripts tiện ích

- [ ] **CI/CD**
  - [ ] GitHub Actions: Rust build + test
  - [ ] GitHub Actions: Flutter build + test
  - [ ] Pre-commit hooks (cargo fmt, clippy)

---

### PHASE 1: Authentication & Core SOS (Tháng 1-2)

#### 1.1 Backend - Auth System
- [ ] Model `User` + migration
- [ ] `POST /api/v1/auth/register` → Gửi OTP qua Twilio
- [ ] `POST /api/v1/auth/verify-otp` → Xác thực OTP, tạo JWT
- [ ] `POST /api/v1/auth/refresh` → Refresh access token
- [ ] JWT middleware (tower layer)
- [ ] Rate limiting middleware

#### 1.2 Backend - User & Contacts
- [ ] Model `EmergencyContact` + migration
- [ ] `GET /api/v1/user/profile`
- [ ] `PUT /api/v1/user/profile`
- [ ] `PUT /api/v1/user/emergency-contacts`
- [ ] `PUT /api/v1/user/settings`
- [ ] Input validation (validator crate)

#### 1.3 Backend - Core SOS Flow
- [ ] Model `SosEvent` + `SosEventLog` + migrations
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
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
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

### 2026-06-17
- **Việc đã làm:**
  - ✅ Phân tích tài liệu PDF kế hoạch dự án (5 trang)
  - ✅ Đánh giá tính khả thi → KHẢ THI
  - ✅ Nhận diện 9 phần thiếu sót và bổ sung đầy đủ
  - ✅ Tạo Implementation Plan chi tiết
  - ✅ Tạo Project Tracker (file này)
- **Vấn đề:**
  - Chưa xác nhận: Tên dự án chính thức (Beacon vs SOS Rescue Ecosystem)
  - Chưa xác nhận: flutter_rust_bridge vs UniFFI
  - Chưa xác nhận: Kinh nghiệm Flutter của developer
  - Chưa xác nhận: Có Twilio/Firebase account chưa
  - Chưa xác nhận: Target Android only hay cả iOS
- **Việc cần làm tiếp:**
  - 🔜 Phase 0: Setup Rust workspace + project structure
  - 🔜 Phase 0: Docker Compose setup
  - 🔜 Phase 0: Database migrations

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

## 🐛 Known Issues & Blockers

> Ghi lại các vấn đề đang chặn tiến độ.

| # | Vấn đề | Mức độ | Trạng thái | Giải pháp |
|---|--------|--------|------------|-----------|
| - | (Chưa có) | - | - | - |

---

## 📚 Tài liệu Tham khảo

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [flutter_rust_bridge](https://cjycode.com/flutter_rust_bridge/)
- [Redis Geospatial](https://redis.io/docs/data-types/geospatial/)
- [Twilio SMS API](https://www.twilio.com/docs/sms)
- [Firebase Cloud Messaging](https://firebase.google.com/docs/cloud-messaging)
- [Nghị định 13/2023/NĐ-CP](https://thuvienphapluat.vn/van-ban/Cong-nghe-thong-tin/Nghi-dinh-13-2023-ND-CP-bao-ve-du-lieu-ca-nhan-357384.aspx)
