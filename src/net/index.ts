import { login } from "./user/login"
import { sendEmailCode, sendSmsCode} from "./user/sendCode"
import { getHitokoto } from "./user/getHitokoto"
import { post } from "./base"
import { get } from "./base"

const userApi = {
    login,
    sendEmailCode,
    sendSmsCode,
    post,
    get,
    getHitokoto
}

export { userApi }