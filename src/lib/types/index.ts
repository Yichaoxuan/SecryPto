// 与 Rust 后端匹配的数据类型

export interface ClipboardEntry {
  id: string;
  content_type: "text" | "image";
  text_content: string | null;
  image_base64: string | null;
  image_thumb: string | null;
  created_at: string;
  is_pinned: boolean;
  expiry_days: number;
  content_hash: string;
}

export interface PasswordEntry {
  id: string;
  name: string;
  url: string;
  username: string;
  encrypted_password: string;
  notes: string;
  tags: string[];
  created_at: string;
  updated_at: string;
}

export interface Settings {
  hotkey: string;
  default_expiry_days: number;
  auto_start: boolean;
  theme: "system" | "light" | "dark";
  language: string;
  vault_locked: boolean;
}
