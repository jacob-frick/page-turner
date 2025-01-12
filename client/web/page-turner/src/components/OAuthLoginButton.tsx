
import { useState } from 'react';

const OAuthLoginButton = () => {
  const [loading, setLoading] = useState(false);
  const [userInfo, setUserInfo] = useState(null);


  const handleLogin = () => {
    const popupWidth = 500;
    const popupHeight = 600;
    const left = window.screenX + (window.outerWidth - popupWidth) / 2;
    const top = window.screenY + (window.outerHeight - popupHeight) / 2;

    // Open the popup window
    const popup = window.open(
      'http://localhost:8080/auth/login', // OAuth login URL from your backend
      'OAuth Login',
      `width=${popupWidth},height=${popupHeight},top=${top},left=${left}`
    );

    if (!popup) {
      alert('Popup blocked! Please enable popups for this site.');
      return;
    }

    setLoading(true);

    // Listen for messages from the popup
    const handleMessage = (event: MessageEvent) => {
      if (event.origin !== 'http://localhost:8080') return; // Ensure origin is trusted

      if (event.data && event.data.type === 'oauth-success') {
        console.log(event.data.userInfo)
        setUserInfo(event.data.userInfo); // Update user info from the popup
        
      }

      if (event.data && event.data.type === 'oauth-failure') {
        console.error('OAuth login failed');
      }

      setLoading(false);
      popup.close();
      window.removeEventListener('message', handleMessage);
      window.location.href = "/dashboard";
    };

    window.addEventListener('message', handleMessage);
  };

  return (
    <div>
      {loading ? (
        <button className="px-4 py-2 bg-gray-500 text-white rounded-lg" disabled>
          Logging in...
        </button>
      ) : (
        <button
          onClick={handleLogin}
          className="px-4 py-2 bg-blue-500 text-white rounded-lg"
        >
          Log in with Google
        </button>
      )}

      {userInfo && (
        <div>
          <h1 className="text-2xl">Hello, {userInfo.name}!</h1>
          <pre>{JSON.stringify(userInfo, null, 2)}</pre>
        </div>
      )}
    </div>
  );
};

export default OAuthLoginButton;
