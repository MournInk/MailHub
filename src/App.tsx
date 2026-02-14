import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Mail } from 'lucide-react';
import './App.css';

function App() {
  const [message, setMessage] = useState('Loading...');

  useEffect(() => {
    // Test backend connection
    invoke('greet', { name: 'MailHub' })
      .then((msg) => setMessage(msg as string))
      .catch((err) => setMessage('Error: ' + err));
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center">
      <div className="text-center">
        <Mail className="w-16 h-16 mx-auto mb-4 text-blue-600" />
        <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-4">
          MailHub
        </h1>
        <p className="text-gray-600 dark:text-gray-400">{message}</p>
        <p className="text-sm text-gray-500 dark:text-gray-500 mt-4">
          One-stop mailbox management system
        </p>
      </div>
    </div>
  );
}

export default App;
