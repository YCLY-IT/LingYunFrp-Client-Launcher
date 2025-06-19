import { Window } from '../types'
import { invoke } from '@tauri-apps/api/core';

const clientVersion = await invoke<string>('get_client_version');
const defaultFailure = (messageText: string) => {
    //! TODO: only console warning, don't show message here
    window.$message?.warning(messageText);
    window.$loadingBar?.error();
};

const defaultError = (err: any) => {
    //! TODO: only console error, don't show message here
    console.error(err);
    window.$message?.error(err.response?.data?.message || err);
    window.$loadingBar?.error();
};

//! TODO: Specifies the params and return value type
function storeToken(Authorization: any, remember: boolean, expires: any) {
    const token = {
        Authorization: Authorization,
        remember: remember,
        expires: expires
    };
    const tokenStr = JSON.stringify(token);
    if (remember) {
        localStorage.setItem('Authorization', tokenStr);
    } else {
        sessionStorage.setItem('Authorization', tokenStr);
    }
}

//! TODO: Specifies the return value type
function getToken() {
    const tokenStr = localStorage.getItem('Authorization') || sessionStorage.getItem('Authorization');
    if (tokenStr) {
        const token = JSON.parse(tokenStr);
        if (token.expires && token.expires < new Date().getTime()) {
            removeToken();
            //! TODO: only return error, don't show message here
            window.$message?.error('登录信息已过期，请重新登录');
            return null;
        }
        return token.Authorization;
    }
    return null;
}

function removeToken() {
    localStorage.removeItem('Authorization');
    sessionStorage.removeItem('Authorization');
}

//! TODO: why the return value has two type(string or Object)?
declare const window: Window

function accessHandle() {

    
    // 在请求开始时显示加载条
    window.$loadingBar?.start()
    
   return {
    'Authorization': `${getToken()}`
};

}

//! TODO: use promise instead of callback
function post(url: string, data: any, headers: Record<string, string | number>, success: Function, failure = defaultFailure, error = defaultError) {
    const postHeaders = {
        ...headers,
        'ClientVersion': clientVersion,
        'Client': 'LingYunFrpClient',
    }
    // 通过 Tauri 后端转发请求
    invoke('forward_request', {
        url: url,
        method: 'POST',
        data: data,
        headers: postHeaders
    }).then((response: any) => {
        if (response.code === 0) {
            success(response);
            window.$loadingBar?.finish();
        } else {
            failure(response.message);
            window.$loadingBar?.error();
        }
    }).catch(err => error(err));
}

//! TODO: use promise instead of callback
function get(url: string, headers: Record<string, string>, success: Function, failure = defaultFailure, error = defaultError) {
    const getHeaders = {
        ...headers,
        'ClientVersion': clientVersion,
        'Client': 'LingYunFrpClient'
    }
    invoke('forward_request', {
        url: url,
        method: 'GET',
        data: {},  // GET 请求通常不需要请求体，但为了保持接口一致，我们传入空对象
        headers: getHeaders
    }).then((response: any) => {
        // 检查是否是完整的URL（外部API）
        if (url.startsWith('http://') || url.startsWith('https://')) {
            // 对于外部API，直接使用返回的数据
            success(response);
        } else {
            // 对于内部API，检查code字段
            if (response.code === 0) {
                success(response);
                window.$loadingBar?.finish();
            } else {
                failure(response.message);
                window.$loadingBar?.error();
            }
        }
    }).catch(err => error(err));
}

function unauthorized() {
    return !getToken();
}


function OpenBrowser(url: string) {
    // 外部浏览器打开
    invoke('open_url', { url: url })
        .then(() => {
            console.log('打开浏览器成功');
        })
        .catch((err) => {
            console.error('打开浏览器失败:', err);
        });
}

export { defaultFailure, defaultError, storeToken, getToken, accessHandle, removeToken, post, get, unauthorized, OpenBrowser }