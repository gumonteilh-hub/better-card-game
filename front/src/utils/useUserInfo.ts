import { useEffect, useState } from "react";
import type { IDeck } from "../types/template";

export interface IUserInfo {
	name?: string;
	deck?: IDeck;
}

export const useUserInfo = () => {
	const [userInfos, setUserInfos] = useState<IUserInfo>();

	useEffect(() => {
		const stringUserInfo = localStorage.getItem("userInfo");
		if (stringUserInfo) {
			try {
				const userInfo = JSON.parse(stringUserInfo) as IUserInfo;
				setUserInfos(userInfo);
			} catch (error) {
				console.error(error);
			}
		}
	}, []);

	const saveUserInfo = (userInfo: IUserInfo) => {
		localStorage.setItem("userInfo", JSON.stringify(userInfo));
		setUserInfos(userInfo);
	};

	return { userInfos, saveUserInfo };
};
