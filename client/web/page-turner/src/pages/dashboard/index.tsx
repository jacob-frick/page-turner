import { useEffect, useState } from "react";
import { useRouter } from "next/router";
import axios, { AxiosError } from "axios";

const Dashboard = () => {
  const [userInfo, setUserInfo] = useState<{email: string } | null>(null);
  const router = useRouter();

  useEffect(() => {
    const fetchUserInfo = async () => {
      try {
        const response = await axios.get("http://localhost:8080/auth/validate_session", {
          withCredentials: true, // Ensure cookies are sent
        });

        // Set user info if successful
        setUserInfo(response.data);
      } catch (err: unknown) {
        if (axios.isAxiosError(err)) {
          if (err.response?.status === 401) {
            router.push("/");
          } else {
            console.error("Error fetching user info:", err.message);
          }
        } else {
          console.error("Unexpected error:", err);
        }
      }
    };

    fetchUserInfo();
  }, [router]);

  if (!userInfo) {
    return <div>Loading...</div>;
  }

  return (
    <div className="p-4">
      <h1 className="text-3xl">Welcome to Your Dashboard</h1>
      <p>
        <strong>Email:</strong> {userInfo.email}
      </p>
    </div>
  );
};

export default Dashboard;
