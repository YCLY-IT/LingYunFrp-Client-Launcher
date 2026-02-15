import { login } from "./user/login";
import { sendEmailCode, sendSmsCode } from "./user/sendCode";
import { getHitokoto } from "./user/getHitokoto";
import { post, get, patch, put, del } from "./base";

const userApi = {
  login,
  sendEmailCode,
  sendSmsCode,
  post,
  get,
  patch,
  put,
  delete: del,
  getHitokoto,
};

export { userApi };
