// pages/dashboard.tsx

import { GetServerSideProps, GetServerSidePropsContext } from 'next';
import { validateSession } from '../../utils/auth';

interface UserInfo {
  email: string;
  first_name: string;
  last_name: string;
  full_name: string;
}

const Dashboard = ({ userInfo }: { userInfo: UserInfo | null }) => {
  if (!userInfo) {
    return <div>Unauthorized</div>;
  }

  return (
    <div>
      <h1>Welcome to your Dashboard</h1>
      <p>Email: {userInfo.email}</p>
      <p>Full Name: {userInfo.full_name}</p>
    </div>
  );
};

export const getServerSideProps: GetServerSideProps = async (context: GetServerSidePropsContext) => {
  const userInfo = await validateSession(context);

  if (!userInfo) {
    return {
      redirect: {
        destination: '/',
        permanent: false,
      },
    };
  }

  return {
    props: {
      userInfo,
    },
  };
};

export default Dashboard;
