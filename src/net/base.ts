import axios from 'axios';
import { Window } from '../types'
import { invoke } from '@tauri-apps/api/core';

const apiUrl = await invoke<string>('api_url');

const updateInfo = await invoke<{
    code: number;
    message: string | null;
    data: {
      current_version: string;
      latest_info: {
        data: any
        version: string;
        download_url: string;
        release_notes: string;
        force_update: boolean;
      };
      needs_update: boolean;
    };
  }>('check_update');

const api = axios.create({
    baseURL: apiUrl,
    headers: {
        //* NOTE: defalut content-type is set
        'Content-Type': 'application/x-www-form-urlencoded',
        'ClientVersion': updateInfo.data.current_version,
        'Client': 'LingYunFrpClient'
    }
});

const defaultFailure = (messageText: string) => {
    //! TODO: only console warning, don't show message here
    window.$message?.warning(messageText);
    window.$loadingBar?.error();
};

const defaultError = (err: any) => {
    //! TODO: only console error, don't show message here
    console.error(err);
    window.$message?.error(err.response.data.message);
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
    api.post(url, data, {
        headers
    }).then(({ data }) => {
        if (data.code === 0) {
            success(data);
        } else {
            failure(data.message);
        }
    }).catch(err => error(err));
}

//! TODO: use promise instead of callback
function get(url: string, headers: Record<string, string>, success: Function, failure = defaultFailure, error = defaultError) {
    api.get(url, {
        headers: headers
    }).then(({ data }) => {
        if (data.code === 0) {
            success(data);
            window.$loadingBar?.finish()
        } else {
            failure(data.message);
            window.$loadingBar?.error()
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

export { api, defaultFailure, defaultError, storeToken, getToken, accessHandle, removeToken, post, get, unauthorized, OpenBrowser }