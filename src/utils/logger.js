import { invoke } from '@tauri-apps/api/core';

/**
 * 日志级别
 */
export const LogLevel = {
  INFO: 'info',
  WARN: 'warn',
  ERROR: 'error'
};

/**
 * 记录信息日志
 * @param {string} message 日志消息
 */
export const logInfo = async (message) => {
  try {
    await invoke('add_log', { level: LogLevel.INFO, message });
  } catch (error) {
    console.error('记录日志失败:', error);
  }
};

/**
 * 记录警告日志
 * @param {string} message 日志消息
 */
export const logWarn = async (message) => {
  try {
    await invoke('add_log', { level: LogLevel.WARN, message });
  } catch (error) {
    console.error('记录日志失败:', error);
  }
};

/**
 * 记录错误日志
 * @param {string} message 日志消息
 */
export const logError = async (message) => {
  try {
    await invoke('add_log', { level: LogLevel.ERROR, message });
  } catch (error) {
    console.error('记录日志失败:', error);
  }
};

/**
 * 获取日志列表
 * @param {string} level 日志级别过滤
 * @param {number} limit 最大返回数量
 * @returns {Promise<Array>} 日志列表
 */
export const getLogs = async (level = null, limit = 100) => {
  try {
    return await invoke('get_logs', { level, limit });
  } catch (error) {
    console.error('获取日志失败:', error);
    return [];
  }
};

/**
 * 清空日志
 * @returns {Promise<boolean>} 是否成功
 */
export const clearLogs = async () => {
  try {
    await invoke('clear_logs');
    return true;
  } catch (error) {
    console.error('清空日志失败:', error);
    return false;
  }
};

/**
 * 创建一个日志记录器
 * @param {string} module 模块名称
 * @returns {Object} 日志记录器对象
 */
export const createLogger = (module) => {
  return {
    info: (message) => logInfo(`[${module}] ${message}`),
    warn: (message) => logWarn(`[${module}] ${message}`),
    error: (message) => logError(`[${module}] ${message}`)
  };
};

// 默认导出日志对象
export default {
  LogLevel,
  logInfo,
  logWarn,
  logError,
  getLogs,
  clearLogs,
  createLogger
}; 