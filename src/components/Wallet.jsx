import { useContext, useEffect, useState } from "react";
import { getDoc } from "@junobuild/core";
import { AuthContext } from "./Auth";

export const Wallet = () => {
  const [address, setAddress] = useState("");

  const { user } = useContext(AuthContext);

  useEffect(() => {
    (async () => {
      if (user === undefined || user === null) {
        return;
      }

      const doc = await getDoc({
        collection: "profiles",
        key: user.key,
      });

      setAddress(doc?.data?.eth_address ?? "");
    })();
  }, [user]);

  return (
    <p className="dark:text-white py-4 md:max-w-lg">
      Wallet address: <output>{address}</output>
    </p>
  );
};
