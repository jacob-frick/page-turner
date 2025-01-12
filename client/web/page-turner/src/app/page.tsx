'use client';
// import { useEffect, useState } from "react";

// const LoginPage = () => {
//   const [userInfo, setUserInfo] = useState<any>(null);
//   const [loading, setLoading] = useState(false);

//   useEffect(() => {
//     // Check if there's a 'code' in the URL (indicating the callback)
//     const urlParams = new URLSearchParams(window.location.search);
//     const code = urlParams.get("code");

//     if (code) {
//       // Once we get the code, we fetch the user info from the backend
//       fetchUserInfo(code);
//     }
//   }, []);

//   const handleLogin = async () => {
//     setLoading(true);

//     // Redirect to the backend's Google OAuth login route
//     const authUrl = "http://localhost:8080/auth/login"; // Your backend OAuth URL
//     window.location.href = authUrl; // Redirect to OAuth login page
//   };

//   const fetchUserInfo = async (code: string) => {
//     try {
//       // Send the code to the backend and fetch user info
//       const res = await fetch(`http://localhost:8080/auth/callback?code=${code}`, {
//         method: 'GET',
//         credentials: 'include', // Include cookies for session management
//       });

//       if (res.ok) {
//         const data = await res.json();
//         setUserInfo(data); // Set user info in state
//         // Redirect to dashboard after successful login
//         window.location.href = "/dashboard";
//       } else {
//         console.error("Failed to fetch user info");
//       }
//     } catch (err) {
//       console.error("Error fetching user info:", err);
//     }
//   };

//   if (loading) {
//     return (
//       <div className="flex justify-center items-center h-screen">
//         <button className="px-4 py-2 bg-blue-500 text-white rounded-lg">
//           Logging in...
//         </button>
//       </div>
//     );
//   }

//   return (
//     <div className="flex justify-center items-center h-screen">
//       {!userInfo ? (
//         <button
//           onClick={handleLogin}
//           className="px-4 py-2 bg-blue-500 text-white rounded-lg"
//         >
//           Log in with Google
//         </button>
//       ) : (
//         <div>
//           <h1 className="text-2xl">Hello World!</h1>
//           <pre>{JSON.stringify(userInfo, null, 2)}</pre>
//         </div>
//       )}
//     </div>
//   );
// };

// export default LoginPage;
import OAuthLoginButton from '../components/OAuthLoginButton';

const LoginPage = () => {
  return (
    <div className="flex justify-center items-center h-screen">
      <OAuthLoginButton />
    </div>
  );
};

export default LoginPage;
