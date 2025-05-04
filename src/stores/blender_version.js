import { create } from "zustand";

export const useDownloadBlenderVersionStore = create((set, get) => ({
  downloadBlenderVersion: [],
  setDownloadBlenderVersion: (newArr) => set({ downloadBlenderVersion: newArr }),
  getDownloadBlenderVersion: () => get().downloadBlenderVersion,
  clearDownloadBlenderVersion: () => set({ downloadBlenderVersion: [] }),

  // installedBlenderVersions: [],
  // setInstalledBlenderVersions
  // getInstalledBlenderVersions

}));
