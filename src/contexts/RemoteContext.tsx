import React, { createContext, useContext, useState } from "react";

interface RemoteContextType {
  selectedRemoteUuid: string | null;
  setSelectedRemote: (remote: string | null) => void;
}

const RemoteContext = createContext<RemoteContextType | undefined>(undefined);

export const RemoteProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [selectedRemoteUuid, setSelectedRemote] = useState<string | null>(null);

  return (
    <RemoteContext.Provider value={{ selectedRemoteUuid, setSelectedRemote }}>
      {children}
    </RemoteContext.Provider>
  );
};

export const useRemoteContext = () => {
  const context = useContext(RemoteContext);
  if (context === undefined) {
    throw new Error("useRemoteContext must be used within a RemoteProvider");
  }
  return context;
};
