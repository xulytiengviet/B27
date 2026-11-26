# B27 — BILA-T27

Nén không mất dữ liệu tiếng Việt | CVNSS4.0 + Base27 + balanced ternary.

## Dùng thử
Node.js 22, không cần npm.
```sh
git clone https://github.com/xulytiengviet/B27.git
cd B27
node b27.cjs c input.txt input.b27
node b27.cjs d input.b27 restored.txt
node test.cjs
node test.cjs corpus.txt
python benchmark.py corpus.txt
```
File đích phải chưa tồn tại; đầu vào tối đa 16 MiB.

## Kiến trúc v0.1
- CVNSS converter 5.0.0-audit-safe: chỉ mã hóa âm tiết chữ thường nếu giải mã chính xác; giữ nguyên mọi phần không thuận nghịch.
- Base27: a–z và space, 3 trit cân bằng; đóng gói 5 ký hiệu / 3 byte.
- Backend DEFLATE, không phải LZ27/Range27 độc lập. Auto chọn raw, DEFLATE gốc hoặc DEFLATE sau biến đổi CVNSS có/không pack.
- B27F: magic 4 byte, mode 1 byte, original length uint32 BE, SHA-256 32 byte, payload. Record: tag 1 byte, length uint32 BE, data. Tag 0 raw, 1 packed, 2 ASCII CVNSS. SHA-256 kiểm tra toàn vẹn, không xác thực nguồn.
- Bản v0.1 cần cùng converter đã cung cấp để giải mã.

## Nghiệm thu
1. 100% byte-for-byte trên bộ test, converter selfTest, 27 ký hiệu, Unicode NFC/NFD, dữ liệu nhị phân giả ngẫu nhiên và checksum lỗi.
2. CI Windows/Linux/macOS; artifact benchmark JSON.
3. So sánh B27-auto, B27-CVNSS, DEFLATE và Brotli trên cùng đầu vào. Python benchmark thêm XZ/LZMA2. Thời gian CLI so với in-process không công bằng; không dùng để chứng minh CPU thấp hơn.
4. Giả thuyết hiệu năng chỉ được chấp nhận khi corpus tiếng Việt độc lập, công bố hash và cấu hình máy, có đối chứng cùng điều kiện và số liệu kích thước, CPU, RAM. Mục tiêu đề xuất: giảm ít nhất 5% kích thước và 10% CPU trên corpus giữ lại so với baseline đã đăng ký trước; nếu không đạt, báo FAIL. Không dùng dữ liệu tổng hợp lặp lại để suy rộng.
5. Không tuyên bố 100% nghiệm thu toàn dự án khi CI hoặc giả thuyết hiệu năng chưa được kiểm chứng.

[Trang giới thiệu (cần bật Pages từ main/root)](https://xulytiengviet.github.io/B27/) · [CI](https://github.com/xulytiengviet/B27/actions)
