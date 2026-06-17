<div align="center">

<img src=".github/assets/banner.png" alt="Beacon - SOS Rescue Ecosystem" width="100%"/>

<br/>

# 🚨 BEACON

### Community-Powered Emergency Rescue Ecosystem

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Flutter](https://img.shields.io/badge/Flutter-02569B?style=for-the-badge&logo=flutter&logoColor=white)](https://flutter.dev/)
[![Axum](https://img.shields.io/badge/Axum-E6522C?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Redis](https://img.shields.io/badge/Redis-DC382D?style=for-the-badge&logo=redis&logoColor=white)](https://redis.io/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)

<br/>

**Trong tình huống khẩn cấp, mỗi giây đều là sự sống còn.**

Beacon biến mọi chiếc điện thoại thành **lá chắn sinh tồn** — một nút bấm duy nhất gửi cảnh báo SOS tới người thân, đồng thời kích hoạt **mạng lưới cứu hộ cộng đồng** trong bán kính 500m.

[📱 Tải App](#-cài-đặt) · [📖 Tài liệu](docs/) · [🐛 Báo lỗi](../../issues) · [💡 Góp ý](../../issues)

</div>

---

## 🌟 Tại sao Beacon khác biệt?

<table>
<tr>
<td width="50%">

### 🔴 Vấn đề hiện tại
- Gọi **113/114/115** mất thời gian chờ, cần mô tả vị trí
- Apps SOS hiện tại chỉ gửi tin nhắn cho người thân — **có thể ở cách xa hàng trăm km**
- Nạn nhân bị khống chế **không thể mở khóa điện thoại**
- Điện thoại bị tịch thu → **mất toàn bộ bằng chứng**

</td>
<td width="50%">

### 🟢 Giải pháp Beacon
- Tự động gửi **tọa độ chính xác** + link Google Maps
- Kích hoạt **radar cộng đồng** — người trong bán kính 500m nhận được cảnh báo
- Hỗ trợ kích hoạt **không cần mở khóa**: BLE button, widget, giọng nói
- Ghi âm tự động + **mã hóa & đẩy lên cloud** ngay lập tức

</td>
</tr>
</table>

---

## ⚡ Tính năng Chính

<div align="center">

```
┌───────────────────────────────────────────────────────────────┐
│                    🚨  SOS ACTIVATED  🚨                     │
│                                                               │
│   📍 GPS Position Locked           🔋 Battery: 67%           │
│   📡 Server Connected              🎙️ Recording: Active      │
│   👨‍👩‍👧‍👦 5 Contacts Notified          📻 Radar: 12 nearby      │
│   ✅ SMS Sent  ✅ Voice Call Made   ✅ Push Alerts Sent       │
│                                                               │
│              ╔═══════════════════════╗                        │
│              ║   ██ CANCEL (PIN) ██  ║                        │
│              ╚═══════════════════════╝                        │
└───────────────────────────────────────────────────────────────┘
```

</div>

### 🔘 Kích hoạt SOS Đa phương thức

| Phương thức | Mô tả | Cần mở khóa? |
|:---:|---|:---:|
| 🔵 **BLE Button** | Nút bấm nhỏ gọn (móc khóa, vòng tay). Nhấn giữ 3s để kích hoạt bí mật từ trong túi | ❌ Không |
| 📱 **Lock Screen Widget** | Widget lớn ngay trên màn hình khóa. Chạm 1 lần duy nhất | ❌ Không |
| 🎤 **Giọng nói** | Cài từ khóa bí mật riêng qua Siri/Google Assistant | ❌ Không |
| 🔴 **Nút In-App** | Nút SOS lớn trên giao diện chính + đếm ngược 5 giây xác nhận | ✅ Có |

### 📡 Radar Cứu hộ Cộng đồng

```
                         🟢 User A (120m)
                        ╱
         🟢 User B ── 🔴 NẠN NHÂN ── 🟢 User C (340m)
          (85m)        (SOS Active)
                        ╲
                         🟢 User D (490m)

   → Tất cả nhận PUSH NOTIFICATION khẩn cấp (vượt chế độ im lặng)
   → Hiển thị khoảng cách + hướng di chuyển tới nạn nhân
```

- **Quét bán kính 500m - 1km** bằng Redis Geospatial (< 2ms response)
- **Push notification cấp độ khẩn cấp** — vượt qua chế độ im lặng/rung
- **Realtime WebSocket** — cập nhật vị trí nạn nhân liên tục

### 🛡️ Bảo vệ & Thu thập Bằng chứng

| Tính năng | Chi tiết |
|---|---|
| 🎙️ **Blackbox Recording** | Tự động ghi âm, mã hóa AES-256 → đẩy lên cloud mỗi 30s. Điện thoại bị phá hủy vẫn còn bằng chứng |
| 📞 **Fake Call Escape** | Cuộc gọi giả thật 100% — giúp thoát tình huống quấy rối mà không gây nghi ngờ |
| 📡 **Offline Fallback** | Mất internet → tự động gửi SMS trực tiếp tới 5 số thân nhân |
| 🔋 **Ultra-Low Battery** | Chế độ chờ < 2% pin/ngày nhờ thuật toán thích ứng chuyển động |

---

## 🏗️ Kiến trúc Hệ thống

```
┌─────────────────────────────────────────────────────────────────────┐
│                        BEACON ARCHITECTURE                         │
│                                                                     │
│  ┌──────────────┐   FFI    ┌──────────────┐   HTTPS   ┌─────────┐ │
│  │  beacon_app  │◄────────►│ beacon_core  │──────────►│ beacon  │ │
│  │              │          │              │           │ _server  │ │
│  │  Flutter UI  │          │  Rust Native │           │          │ │
│  │  • Screens   │          │  • GPS       │   WSS     │  Axum   │ │
│  │  • Widgets   │          │  • BLE       │◄─────────►│  REST   │ │
│  │  • Services  │          │  • Crypto    │           │  WS     │ │
│  │  • State     │          │  • Audio     │           │  Auth   │ │
│  └──────────────┘          │  • Network   │           └────┬────┘ │
│                            │  • SMS       │                │      │
│                            └──────────────┘                │      │
│                                                            │      │
│  ┌──────────────┐                              ┌──────────┴─────┐ │
│  │beacon_shared │                              │   Data Layer   │ │
│  │  Rust Types  │                              │ PostgreSQL 16  │ │
│  │  Constants   │                              │ Redis 7 (Geo)  │ │
│  └──────────────┘                              │ S3 (Objects)   │ │
│                                                └────────────────┘ │
│                                                                     │
│  External Services: Twilio (SMS/Voice) · Firebase (FCM) · Maps    │
└─────────────────────────────────────────────────────────────────────┘
```

### Tại sao Rust?

> **Đây là ứng dụng liên quan trực tiếp đến tính mạng con người.** Mọi crash, memory leak, hay bottleneck đều có thể là sự khác biệt giữa sống và chết.

| Tiêu chí | Rust | Alternatives |
|---|---|---|
| **Memory Safety** | ✅ Zero-crash guarantee (no GC, no null pointers) | ❌ Java/Kotlin: GC pauses, OOM |
| **Concurrency** | ✅ 100k+ concurrent connections (tokio) | ⚠️ Node.js: single-threaded |
| **Battery** | ✅ Native performance, no runtime overhead | ❌ React Native: JS bridge overhead |
| **Latency** | ✅ Sub-millisecond SOS processing | ⚠️ Go: GC stop-the-world pauses |
| **Server Cost** | ✅ 10x less memory than Java/Node | ❌ Higher cloud bills |

---

## 📁 Cấu trúc Dự án

```
SOS_Rescue_Ecosystem/
│
├── beacon_app/                 # 📱 Flutter Mobile Application
│   ├── lib/
│   │   ├── core/               #    DI, routing, theme, constants
│   │   ├── features/           #    Feature modules (auth, home, radar, ...)
│   │   ├── services/           #    API, notification, location services
│   │   └── widgets/            #    Reusable UI components
│   ├── android/                #    Android-specific config
│   └── ios/                    #    iOS-specific config
│
├── beacon_core/                # ⚙️ Rust Core Library (Mobile Native)
│   └── src/
│       ├── gps/                #    GPS tracking & motion detection
│       ├── ble/                #    Bluetooth LE device management
│       ├── crypto/             #    AES-256-GCM encryption
│       ├── audio/              #    Blackbox recording
│       ├── network/            #    HTTP/WS client & offline queue
│       └── sms/                #    SMS fallback (Android)
│
├── beacon_server/              # 🖥️ Rust Backend Server (Axum)
│   └── src/
│       ├── routes/             #    API endpoints (auth, sos, radar, ...)
│       ├── models/             #    Database models (SQLx)
│       ├── services/           #    Business logic layer
│       ├── middleware/         #    JWT auth, rate limiting, CORS
│       └── websocket/          #    Realtime event streaming
│
├── beacon_shared/              # 📦 Shared Rust Types & Constants
│
├── migrations/                 # 🗄️ PostgreSQL migrations (SQLx)
├── docs/                       # 📖 Documentation
├── docker-compose.yml          # 🐳 PostgreSQL + Redis + Server
└── .github/                    # 🔧 CI/CD workflows
```

---

## 🚀 Bắt đầu Nhanh

### Yêu cầu Hệ thống

| Tool | Version | Mục đích |
|------|---------|----------|
| [Rust](https://rustup.rs/) | stable (≥ 1.75) | Backend + Core |
| [Flutter](https://flutter.dev/docs/get-started/install) | ≥ 3.19 | Mobile app |
| [Docker](https://www.docker.com/) | ≥ 24.0 | PostgreSQL + Redis |
| [sqlx-cli](https://crates.io/crates/sqlx-cli) | latest | Database migrations |

### Cài đặt & Chạy

```bash
# 1. Clone repository
git clone https://github.com/user/SOS_Rescue_Ecosystem.git
cd SOS_Rescue_Ecosystem

# 2. Khởi động databases
docker compose up -d

# 3. Setup environment
cp .env.example .env
# → Chỉnh sửa .env với Twilio API keys, database URL, etc.

# 4. Chạy database migrations
sqlx migrate run

# 5. Khởi động Backend Server
cargo run -p beacon_server
# → Server running at http://localhost:3000

# 6. Chạy Flutter App (terminal khác)
cd beacon_app
flutter pub get
flutter run
```

### Biến Môi trường

```env
# Database
DATABASE_URL=postgres://beacon:beacon@localhost:5432/beacon_db
REDIS_URL=redis://localhost:6379

# Authentication
JWT_SECRET=your-256-bit-secret-key
JWT_EXPIRY_HOURS=24
OTP_EXPIRY_SECONDS=300

# Twilio
TWILIO_ACCOUNT_SID=ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
TWILIO_AUTH_TOKEN=your_auth_token
TWILIO_PHONE_NUMBER=+1234567890

# Firebase
FIREBASE_PROJECT_ID=beacon-sos
FIREBASE_SERVICE_ACCOUNT_KEY=path/to/serviceAccountKey.json

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=beacon_server=debug,tower_http=debug
```

---

## 🔒 Bảo mật & Quyền riêng tư

Beacon được thiết kế với nguyên tắc **Privacy by Design**:

| Biện pháp | Chi tiết |
|---|---|
| 🔐 **Mã hóa đầu cuối** | Bản ghi âm được mã hóa AES-256-GCM ngay trên thiết bị trước khi upload |
| 🕵️ **Vị trí ẩn danh** | Radar cộng đồng sử dụng hashed user ID — không ai biết danh tính bạn |
| 🔑 **JWT Authentication** | Access token (ngắn hạn) + Refresh token (dài hạn) |
| 🛡️ **PIN bảo mật** | Hủy SOS bắt buộc nhập PIN — ngăn kẻ xấu tắt cảnh báo |
| 🗑️ **Auto-purge** | Dữ liệu vị trí tự động xóa sau 24h, bản ghi âm sau 30 ngày |
| 📜 **Tuân thủ pháp luật** | Phù hợp Nghị định 13/2023/NĐ-CP về bảo vệ dữ liệu cá nhân |
| 🔍 **Mã nguồn mở** | Core module công khai — cộng đồng có thể audit bất cứ lúc nào |

---

## 📊 Hiệu năng

Được xây dựng để hoạt động khi **cần nhất** — ngay cả trong thảm họa diện rộng:

```
┌──────────────────────────────────────────────────┐
│          BENCHMARK (Production Target)           │
│                                                  │
│  ⚡ SOS → Server → Notify:      < 500ms         │
│  📡 Radar Geosearch (500m):     < 2ms            │
│  🔌 Concurrent Connections:     100,000+         │
│  🔋 Background Battery/Day:    < 2%              │
│  💾 Server Memory (10k users):  < 256MB          │
│  📦 App Size (APK):            < 25MB            │
│                                                  │
└──────────────────────────────────────────────────┘
```

---

## 🗺️ Lộ trình Phát triển

```
  Q1 2026          Q2 2026          Q3 2026         Q4 2026
    │                 │                │               │
    ▼                 ▼                ▼               ▼
 ┌──────┐        ┌──────────┐    ┌──────────┐    ┌─────────┐
 │ MVP  │───────►│Community │───►│ Advanced │───►│ Release │
 │ Core │        │  Radar   │    │ Features │    │  v1.0   │
 └──────┘        └──────────┘    └──────────┘    └─────────┘
    │                 │                │               │
 • SOS Button     • Geospatial     • BLE Button    • Stress Test
 • GPS Track       Radar          • Blackbox       • Security
 • SMS/Call      • Push Alerts      Recording       Audit
 • Auth          • Battery Opt   • Fake Call      • Play Store
 • 5 Contacts    • Widgets       • Voice SOS      • App Store
```

---

## 🤝 Đóng góp

Beacon là dự án **mã nguồn mở** vì chúng tôi tin rằng **an toàn cộng đồng không nên là đặc quyền**. Mọi đóng góp đều được trân trọng!

### Cách đóng góp

```bash
# 1. Fork repository
# 2. Tạo branch tính năng
git checkout -b feature/ten-tinh-nang

# 3. Commit changes
git commit -m "feat: mô tả tính năng"

# 4. Push & tạo Pull Request
git push origin feature/ten-tinh-nang
```

### Quy ước Commit Message

```
feat:     Tính năng mới
fix:      Sửa lỗi
docs:     Cập nhật tài liệu
style:    Format code (không thay đổi logic)
refactor: Tái cấu trúc code
test:     Thêm/sửa test
perf:     Cải thiện hiệu năng
ci:       CI/CD configuration
```

### Cần giúp đỡ ở đâu?

- 🐛 **Bug Reports** — Tìm và báo lỗi
- 📖 **Documentation** — Cải thiện tài liệu
- 🌐 **Localization** — Dịch app sang ngôn ngữ khác
- 🧪 **Testing** — Viết tests, test trên thiết bị thật
- 🎨 **UI/UX** — Cải thiện giao diện và trải nghiệm
- 🔒 **Security** — Audit bảo mật, penetration testing

---

## 📄 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for more information.

```
MIT License — Bạn được tự do sử dụng, sao chép, chỉnh sửa và phân phối
phần mềm này cho bất kỳ mục đích nào, kể cả thương mại.
```

---

## 📬 Liên hệ

- **Issues:** [GitHub Issues](../../issues)
- **Discussions:** [GitHub Discussions](../../discussions)

---

<div align="center">

### 💙 Beacon — Vì mỗi giây đều quan trọng

*Được xây dựng bằng ❤️ và Rust 🦀 cho cộng đồng Việt Nam*

**Nếu dự án hữu ích, hãy cho chúng tôi một ⭐ trên GitHub!**

</div>
