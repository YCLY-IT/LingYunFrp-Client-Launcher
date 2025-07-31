// 天气服务工具类
import { get } from "../base";

export interface WeatherInfo {
  weather: string;
  temp: string;
  humidity: string;
  winddirection: string;
  windpower: string;
  reporttime: string;
}

export interface LocationInfo {
  ip: string;
  province: string;
  city: string;
  adcode?: string;
}

// 缓存接口
interface CacheItem<T> {
  data: T;
  timestamp: number;
  expiresIn: number; // 缓存过期时间（毫秒）
}

class WeatherService {
  private locationCache: Map<string, CacheItem<LocationInfo>> = new Map();
  private weatherCache: Map<string, CacheItem<WeatherInfo>> = new Map();
  private ipCache: CacheItem<string> | null = null;

  // 缓存配置
  private readonly LOCATION_CACHE_DURATION = 30 * 60 * 1000; // 位置信息缓存30分钟
  private readonly WEATHER_CACHE_DURATION = 10 * 60 * 1000; // 天气信息缓存10分钟
  private readonly IP_CACHE_DURATION = 60 * 60 * 1000; // IP信息缓存1小时

  /**
   * 检查缓存是否有效
   */
  private isCacheValid<T>(cacheItem: CacheItem<T> | null): boolean {
    if (!cacheItem) return false;
    return Date.now() - cacheItem.timestamp < cacheItem.expiresIn;
  }

  /**
   * 获取IP地址和位置信息
   */
  async getLocationInfo(ip?: string): Promise<LocationInfo> {
    try {
      // 如果没有提供IP，使用当前访问者的IP
      const targetIp = ip || (await this.getCurrentIP());

      // 检查缓存
      const cachedLocation = this.locationCache.get(targetIp);
      if (this.isCacheValid(cachedLocation)) {
        console.log(`使用缓存的位置信息: ${targetIp}`);
        return cachedLocation!.data;
      }

      // 如果是本地IP，直接返回默认位置
      if (
        targetIp === "127.0.0.1" ||
        targetIp.startsWith("192.168.") ||
        targetIp.startsWith("10.")
      ) {
        const defaultLocation: LocationInfo = {
          ip: targetIp,
          province: "北京市",
          city: "北京市",
          adcode: "beijing",
        };

        // 缓存默认位置信息
        this.locationCache.set(targetIp, {
          data: defaultLocation,
          timestamp: Date.now(),
          expiresIn: this.LOCATION_CACHE_DURATION,
        });

        return defaultLocation;
      }

      // 使用ip-api.com获取位置信息（免费且无需密钥）
      return new Promise<LocationInfo>((resolve, reject) => {
        get(
          `http://ip-api.com/json/${targetIp}?lang=zh-CN`,
          {},
          (data: any) => {
            if (data.status === "success") {
              const locationInfo: LocationInfo = {
                ip: targetIp,
                province: data.regionName || "未知",
                city: data.city || "未知",
                adcode: data.city || "", // 使用城市名作为标识
              };

              // 缓存位置信息
              this.locationCache.set(targetIp, {
                data: locationInfo,
                timestamp: Date.now(),
                expiresIn: this.LOCATION_CACHE_DURATION,
              });

              resolve(locationInfo);
            } else {
              reject(new Error("位置信息获取失败"));
            }
          },
          (message: string) => {
            reject(new Error(message));
          },
          (error: any) => {
            reject(error);
          },
        );
      });
    } catch (error) {
      console.error("获取位置信息失败:", error);
      const fallbackLocation: LocationInfo = {
        ip: ip || "未知",
        province: "未知",
        city: "未知",
      };

      // 缓存错误结果，避免频繁重试
      if (ip) {
        this.locationCache.set(ip, {
          data: fallbackLocation,
          timestamp: Date.now(),
          expiresIn: 5 * 60 * 1000, // 错误结果缓存5分钟
        });
      }

      return fallbackLocation;
    }
  }

  /**
   * 获取天气信息
   */
  async getWeatherInfo(location: string): Promise<WeatherInfo> {
    try {
      // 检查缓存
      const cachedWeather = this.weatherCache.get(location);
      if (this.isCacheValid(cachedWeather)) {
        console.log(`使用缓存的天气信息: ${location}`);
        return cachedWeather!.data;
      }

      // 使用wttr.in API获取天气信息（免费且无需密钥）
      return new Promise<WeatherInfo>((resolve, reject) => {
        get(
          `https://wttr.in/${encodeURIComponent(location)}?format=j1`,
          {},
          (data: any) => {
            if (data.current_condition && data.current_condition.length > 0) {
              const current = data.current_condition[0];
              const windDirMap: Record<string, string> = {
                N: "北风",
                NNE: "北东北风",
                NE: "东北风",
                ENE: "东东北风",
                E: "东风",
                ESE: "东东南风",
                SE: "东南风",
                SSE: "南东南风",
                S: "南风",
                SSW: "南西南风",
                SW: "西南风",
                WSW: "西西南风",
                W: "西风",
                WNW: "西西北风",
                NW: "西北风",
                NNW: "北西北风",
              };

              // 天气描述映射表
              const weatherMap: Record<string, string> = {
                Sunny: "晴",
                Clear: "晴",
                "Partly cloudy": "多云",
                Cloudy: "阴",
                Overcast: "阴",
                Mist: "雾",
                Fog: "雾",
                "Light rain": "小雨",
                "Moderate rain": "中雨",
                "Heavy rain": "大雨",
                "Light snow": "小雪",
                "Moderate snow": "中雪",
                "Heavy snow": "大雪",
                Thunderstorm: "雷阵雨",
                Shower: "阵雨",
                Drizzle: "毛毛雨",
                Haze: "霾",
                Dust: "沙尘",
                Sand: "沙尘",
                Ash: "火山灰",
                Squall: "狂风",
                Tornado: "龙卷风",
                "Tropical storm": "热带风暴",
                Hurricane: "飓风",
                Cold: "寒冷",
                Hot: "炎热",
                Windy: "大风",
                Hail: "冰雹",
                Calm: "无风",
                "Light breeze": "微风",
                "Gentle breeze": "微风",
                "Moderate breeze": "和风",
                "Fresh breeze": "清风",
                "Strong breeze": "强风",
                "High wind": "大风",
                Gale: "狂风",
                "Severe gale": "烈风",
                Storm: "暴风",
                "Violent storm": "狂风",
                Typhoon: "台风",
                "Patchy rain nearby": "局部有雨",
              };

              // 获取天气描述，优先使用中文，如果没有则映射英文
              let weatherDesc =
                current.lang_zh?.[0]?.value ||
                current.weatherDesc?.[0]?.value ||
                "未知";

              // 如果是英文描述，尝试映射为中文
              if (weatherDesc && !/[\u4e00-\u9fa5]/.test(weatherDesc)) {
                weatherDesc = weatherMap[weatherDesc] || weatherDesc;
              }

              const weatherInfo: WeatherInfo = {
                weather: weatherDesc,
                temp: current.temp_C || "--",
                humidity: current.humidity || "--",
                winddirection:
                  windDirMap[current.winddir16Point] ||
                  current.winddir16Point ||
                  "--",
                windpower: current.windspeedKmph
                  ? `${current.windspeedKmph}级`
                  : "--",
                reporttime:
                  current.observation_time || new Date().toLocaleString(),
              };

              // 缓存天气信息
              this.weatherCache.set(location, {
                data: weatherInfo,
                timestamp: Date.now(),
                expiresIn: this.WEATHER_CACHE_DURATION,
              });

              resolve(weatherInfo);
            } else {
              reject(new Error("天气数据为空"));
            }
          },
          (message: string) => {
            reject(new Error(message));
          },
          (error: any) => {
            reject(error);
          },
        );
      });
    } catch (error) {
      console.error("获取天气信息失败:", error);
      // 返回默认天气信息
      const defaultWeather: WeatherInfo = {
        weather: "晴",
        temp: "25",
        humidity: "60",
        winddirection: "东",
        windpower: "3",
        reporttime: new Date().toLocaleString(),
      };

      // 缓存错误结果，避免频繁重试
      this.weatherCache.set(location, {
        data: defaultWeather,
        timestamp: Date.now(),
        expiresIn: 5 * 60 * 1000, // 错误结果缓存5分钟
      });

      return defaultWeather;
    }
  }

  /**
   * 获取当前IP地址
   */
  private async getCurrentIP(): Promise<string> {
    try {
      // 检查IP缓存
      if (this.isCacheValid(this.ipCache)) {
        console.log("使用缓存的IP地址");
        return this.ipCache!.data;
      }

      // 只保留可用的IP查询服务
      const services = ["https://ipinfo.io/json"];

      for (const service of services) {
        try {
          const ip = await new Promise<string>((resolve, reject) => {
            get(
              service,
              {},
              (data: any) => {
                if (data.ip) {
                  resolve(data.ip);
                } else {
                  reject(new Error("IP查询服务返回数据格式错误"));
                }
              },
              (message: string) => {
                reject(new Error(message));
              },
              (error: any) => {
                reject(error);
              },
            );
          });

          // 缓存IP地址
          this.ipCache = {
            data: ip,
            timestamp: Date.now(),
            expiresIn: this.IP_CACHE_DURATION,
          };

          return ip;
        } catch (error) {
          console.warn(`IP查询服务 ${service} 失败:`, error);
          continue;
        }
      }

      // 如果所有服务都失败，返回本地IP
      console.warn("所有IP查询服务都失败，使用本地IP");
      const localIP = "127.0.0.1";

      // 缓存本地IP
      this.ipCache = {
        data: localIP,
        timestamp: Date.now(),
        expiresIn: this.IP_CACHE_DURATION,
      };

      return localIP;
    } catch (error) {
      console.error("获取当前IP失败:", error);
      const localIP = "127.0.0.1";

      // 缓存错误结果
      this.ipCache = {
        data: localIP,
        timestamp: Date.now(),
        expiresIn: 5 * 60 * 1000, // 错误结果缓存5分钟
      };

      return localIP;
    }
  }

  /**
   * 获取完整的访问者信息（位置+天气）
   */
  async getVisitorInfo(ip?: string): Promise<{
    location: LocationInfo;
    weather: WeatherInfo;
  }> {
    const location = await this.getLocationInfo(ip);
    let weather: WeatherInfo;

    if (location.city && location.city !== "未知") {
      weather = await this.getWeatherInfo(location.city);
    } else {
      // 如果没有城市信息，使用默认天气
      weather = {
        weather: "晴",
        temp: "25",
        humidity: "60",
        winddirection: "东",
        windpower: "3",
        reporttime: new Date().toLocaleString(),
      };
    }

    return { location, weather };
  }

  /**
   * 清除所有缓存
   */
  clearCache(): void {
    this.locationCache.clear();
    this.weatherCache.clear();
    this.ipCache = null;
    console.log("已清除所有缓存");
  }

  /**
   * 清除指定IP的位置缓存
   */
  clearLocationCache(ip?: string): void {
    if (ip) {
      this.locationCache.delete(ip);
      console.log(`已清除IP ${ip} 的位置缓存`);
    } else {
      this.locationCache.clear();
      console.log("已清除所有位置缓存");
    }
  }

  /**
   * 清除指定城市的天气缓存
   */
  clearWeatherCache(city?: string): void {
    if (city) {
      this.weatherCache.delete(city);
      console.log(`已清除城市 ${city} 的天气缓存`);
    } else {
      this.weatherCache.clear();
      console.log("已清除所有天气缓存");
    }
  }

  /**
   * 获取缓存统计信息
   */
  getCacheStats(): {
    locationCacheSize: number;
    weatherCacheSize: number;
    hasIpCache: boolean;
  } {
    return {
      locationCacheSize: this.locationCache.size,
      weatherCacheSize: this.weatherCache.size,
      hasIpCache: this.ipCache !== null,
    };
  }
}

export const weatherService = new WeatherService();
