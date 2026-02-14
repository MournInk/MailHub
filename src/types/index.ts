export interface EmailAccount {
  id: string;
  name: string;
  email: string;
  displayName?: string;
  tags?: string[];
  protocol: 'imap' | 'pop3' | 'oauth2';
  provider?: 'gmail' | 'outlook' | 'other';
  config: {
    host?: string;
    port?: number;
    username?: string;
    password?: string;
    oauthToken?: string;
    refreshToken?: string;
  };
}

export interface Email {
  id: string;
  accountId: string;
  subject: string;
  from: { name?: string; address: string };
  to: { name?: string; address: string }[];
  cc?: { name?: string; address: string }[];
  bcc?: { name?: string; address: string }[];
  date: Date;
  body: string;
  htmlBody?: string;
  attachments?: Attachment[];
  isRead: boolean;
  isStarred: boolean;
  labels?: string[];
  aiClassification?: AIClassification;
}

export interface Attachment {
  id: string;
  filename: string;
  mimeType: string;
  size: number;
  content?: string;
}

export interface AIClassification {
  category: 'marketing' | 'important' | 'verification' | 'normal';
  verificationCode?: string;
  verificationLink?: string;
  shouldNotify: boolean;
}

export interface AIConfig {
  enabled: boolean;
  provider: 'openai' | 'anthropic' | 'gemini';
  apiKey: string;
  apiEndpoint?: string;
  model?: string;
  autoDelete: boolean;
}

export interface AppSettings {
  notifications: boolean;
  aiConfig?: AIConfig;
  theme: 'light' | 'dark' | 'system';
}
