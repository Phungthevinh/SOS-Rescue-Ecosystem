# 🚨 Beacon App (Flutter Client)

> Đây là ứng dụng người dùng cuối (Frontend Mobile/Desktop) thuộc dự án **Beacon - SOS Rescue Ecosystem**. 

Beacon App được xây dựng bằng **Flutter**, đóng vai trò là giao diện tương tác chính cho người dùng. Nó chịu trách nhiệm hiển thị UI/UX, quản lý trạng thái hiển thị, và giao tiếp với nhân lõi hệ thống (`beacon_core` viết bằng Rust) thông qua `flutter_rust_bridge`.

---

## ✨ Các Tính Năng Chính (Theo Lộ Trình)

*   **Authentication & SOS Khẩn cấp:** Đăng nhập an toàn (OTP, JWT), kích hoạt SOS bằng một chạm.
*   **Community Radar:** Quét và cảnh báo vị trí ẩn danh trong bán kính an toàn.
*   **Tích hợp Phần Cứng (Tương lai):** Giao tiếp với nút bấm Bluetooth LE ngoài, tự động ghi âm (Blackbox).
*   **Hoạt động Nền (Background):** Tối ưu hóa pin, lấy vị trí ngay cả khi app đang tắt, và cảnh báo qua Push Notification.

---

## 🏗️ Kiến Trúc Ứng Dụng

Ứng dụng tuân theo mô hình phân tách logic chặt chẽ:

*   **Flutter (UI Layer):** Tập trung hoàn toàn vào việc vẽ giao diện, animations, widget lockscreen, bản đồ, và nhận thao tác người dùng.
*   **Rust (Core Logic):** Toàn bộ các xử lý nặng, mã hóa (Encryption), tối ưu pin, kết nối BLE, lưu trữ offline SQLite, và gọi mạng được ủy thác cho thư viện Rust `beacon_core`.
*   **Giao tiếp (Bridge):** Sử dụng `flutter_rust_bridge` để gọi hàm hai chiều giữa Dart và Rust một cách nhanh chóng, an toàn.

---

## ⚙️ Yêu Cầu Môi Trường (Prerequisites)

Để biên dịch và phát triển ứng dụng này, bạn cần cài đặt:

1.  **Flutter SDK** (Phiên bản >= 3.x).
2.  **Rust Toolchain** (để biên dịch `beacon_core` đi kèm).
3.  **Android Studio** (cho phát triển và máy ảo Android) hoặc **Xcode** (cho iOS/macOS).
4.  **C/C++ Build Tools** (nếu biên dịch cho Windows Desktop).

---

## 🚀 Hướng Dẫn Chạy Dự Án (Getting Started)

**1. Khôi phục thư viện phụ thuộc (Dependencies)**
Di chuyển vào thư mục `beacon_app` và chạy lệnh tải package:
```bash
cd beacon_app
flutter pub get
```

**2. Biên dịch mã Rust (Cần thiết trước khi chạy Flutter)**
*(Lưu ý: Quá trình này sẽ được hướng dẫn chi tiết sau khi `flutter_rust_bridge` được tích hợp)*

**3. Khởi chạy ứng dụng**
Chạy ứng dụng trên thiết bị đang kết nối hoặc thiết bị mô phỏng:
```bash
# Chạy trên thiết bị mặc định
flutter run

# Chạy cụ thể trên Windows (nếu có hỗ trợ)
flutter run -d windows

# Chạy trên Chrome (Web)
flutter run -d chrome
```

---

## 📁 Cấu Trúc Thư Mục `lib/`

*(Đang trong quá trình xây dựng)*

```text
lib/
├── main.dart             # Điểm bắt đầu của ứng dụng
├── src/
│   ├── core/             # Cấu hình app, theme, constants, router
│   ├── features/         # Các chức năng chính (auth, sos, radar, settings)
│   ├── shared/           # Các widget dùng chung (buttons, dialogs)
│   └── rust/             # File Dart được sinh tự động từ flutter_rust_bridge
```

---

## 📚 Tài Liệu Hữu Ích

- [Tài liệu chính thức của Flutter](https://docs.flutter.dev/)
- [Tài liệu Flutter Rust Bridge (v2)](https://fzyzcjy.github.io/flutter_rust_bridge/manual/)
- Xem tài liệu tổng quan dự án tại file `PROJECT_TRACKER.md` ở thư mục gốc.
