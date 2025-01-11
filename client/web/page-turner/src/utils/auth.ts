// utils/auth.ts

import { GetServerSidePropsContext } from 'next';

interface UserInfo {
  email: string;
  first_name: string;
  last_name: string;
  full_name: string;
}

export const validateSession = async (context: GetServerSidePropsContext): Promise<UserInfo | null> => {
  try {
    const cookies = context.req.headers.cookie || '';
    const sessionId = parseCookies(cookies).custom_session_id;  // Extract your session ID

    if (!sessionId) {
      throw new Error('Session ID not found');
    }

    const response = await fetch('http://localhost:8080/auth/validate_session', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'Cookie': `custom_session_id=${sessionId}`,  // Send the cookie in the request
      },
    });

    if (!response.ok) {
      throw new Error('Unauthorized');
    }

    const data: UserInfo = await response.json();
    return data; // Return user info if session is valid

  } catch (err) {
    console.error('Error during session validation:', err);
    return null;  // Return null if any error occurs
  }
};

// Utility function to parse cookies
const parseCookies = (cookieHeader: string): Record<string, string> => {
  return cookieHeader.split(';').reduce((acc: Record<string, string>, cookie: string) => {
    const [name, value] = cookie.trim().split('=');
    acc[name] = value;
    return acc;
  }, {});
};
