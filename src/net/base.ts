import { Window } from '../types'
import { invoke } from '@tauri-apps/api/core';

const clientVersion = await invoke<string>('get_client_version');

// 消息去重机制
class MessageDeduplicator {
    private static instance: MessageDeduplicator;
    private messageQueue: Map<string, number> = new Map();
    private readonly DEBOUNCE_TIME = 4000;

    static getInstance(): MessageDeduplicator {
        if (!MessageDeduplicator.instance) {
            MessageDeduplicator.instance = new MessageDeduplicator();
        }
        return MessageDeduplicator.instance;
    }

    showMessage(message: string, type: 'error' | 'warning' | 'success' | 'info' = 'error'): void {
        const now = Date.now();
        const key = `${type}:${message}`;
        
        // 检查是否在去重时间窗口内
        const lastTime = this.messageQueue.get(key);
        if (lastTime && (now - lastTime) < this.DEBOUNCE_TIME) {
            return; // 跳过重复消息
        }
        
        // 更新消息时间戳
        this.messageQueue.set(key, now);
        
        // 显示消息
        switch (type) {
            case 'error':
                window.$message?.error(message);
                break;
            case 'warning':
                window.$message?.warning(message);
                break;
            case 'success':
                window.$message?.success(message);
                break;
            case 'info':
                window.$message?.info(message);
                break;
        }
    }

    // 清理过期的消息记录
    cleanup(): void {
        const now = Date.now();
        for (const [key, time] of this.messageQueue.entries()) {
            if ((now - time) > this.DEBOUNCE_TIME) {
                this.messageQueue.delete(key);
            }
        }
    }
}

const messageDeduplicator = MessageDeduplicator.getInstance();

// 定期清理过期消息记录
setInterval(() => {
    messageDeduplicator.cleanup();
}, 5000);

const defaultFailure = (messageText: string) => {
    //! TODO: only console warning, don't show message here
    messageDeduplicator.showMessage(messageText, 'warning');
    window.$loadingBar?.error();
};

const defaultError = (err: any) => {
    //! TODO: only console error, don't show message here
    console.error(err);
    if (err.response) {
        if (err.response.data.code === 2) {
            window.$dialog?.error({
                title: '提示',
                content: '登录信息已过期，请重新登录',
                positiveText: '确定',
                negativeText: '取消',
                onPositiveClick: () => {
                    removeToken();
                    window.location.href = '/login';
                }
            });
        }
    }
    window.$message?.error(err.response?.data?.message || '请求失败，网络可能存在问题');
    window.$loadingBar?.error()
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
            messageDeduplicator.showMessage('登录信息已过期，请重新登录', 'error');
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
    return {
        'Authorization': getToken()
    };

}

//! TODO: use promise instead of callback
function post(url: string, data: any, headers: Record<string, string | number>, success: Function, failure = defaultFailure, error = defaultError) {
    window.$loadingBar?.start();
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
    }).then((data: any) => {
        if (data.code === 0) {
            success(data);
            window.$loadingBar?.finish();
        } else if (data.code === 2) {
            window.$dialog?.error({
                title: '提示',
                content: '登录信息已过期，请重新登录',
                positiveText: '确定',
                negativeText: '取消',
                onPositiveClick: () => {
                    removeToken();
                    window.location.href = '/login';
                }
            });
            
            failure(data.message);
            window.$loadingBar?.error();
        } else if (data.code === 1) {
            failure(data.message);
            window.$loadingBar?.error();
        }
    }).catch(err => {
        error(err);
    });
}

//! TODO: use promise instead of callback
function get(url: string, headers: Record<string, string>, success: Function, failure = defaultFailure, error = defaultError) {
    window.$loadingBar?.start();
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
    }).then((data: any) => {
        // 检查是否是完整的URL（外部API）
        if (url.startsWith('http://') || url.startsWith('https://')) {
            window.$loadingBar?.finish()
            success(data);
        } else {
            if (data.code === 0) {
                window.$loadingBar?.finish()
                success(data);
            }else if (data.code === 2) {
                window.$dialog?.error({
                    title: '提示',
                    content: '登录信息已过期，请重新登录',
                    positiveText: '确定',
                    negativeText: '取消',
                    onPositiveClick: () => {
                        removeToken();
                        window.location.href = '/login';
                    }
                });
                failure(data.message); 
                window.$loadingBar?.error();
            } else if (data.code === 1) {
                window.$loadingBar?.error();
                failure(data.message);
            }
        }
    }).catch(err => {
        error(err);
    });
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