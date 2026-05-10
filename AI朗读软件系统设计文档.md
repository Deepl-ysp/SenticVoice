# AI朗读软件系统设计文档

**版本**: v1.0  
**日期**: 2025-05-10  
**作者**: SOLO AI Assistant

---

## 目录

1. [项目概述](#1-项目概述)
2. [系统架构设计](#2-系统架构设计)
3. [自制TTS引擎设计](#3-自制tts引擎设计)
4. [情绪控制文件(EMOD)设计](#4-情绪控制文件emod设计)
5. [自动音频标注系统设计](#5-自动音频标注系统设计)
6. [前端架构设计](#6-前端架构设计)
7. [后端架构设计](#7-后端架构设计)
8. [API接口规范](#8-api接口规范)
9. [数据流与交互流程](#9-数据流与交互流程)
10. [部署与运维](#10-部署与运维)
11. [技术选型总结](#11-技术选型总结)

---

## 1. 项目概述

### 1.1 项目目标

构建一套完整的AI朗读软件系统，具备以下核心能力：

| 功能模块 | 描述 |
|---------|------|
| 音色模型训练 | 支持用户上传音频数据，训练个性化音色模型 |
| 模型加载管理 | 加载已训练的模型文件(.pth, .index, .emod) |
| 文本转语音 | 使用自制TTS引擎将文本转换为高质量语音 |
| 情绪控制 | 通过EMOD文件精确控制朗读情绪表达 |
| 自动标注 | 训练音频自动文本标注，支持手动修正 |

### 1.2 核心技术约束

- **前端**: 纯HTML/CSS/JavaScript模块化开发，无第三方框架依赖
- **后端**: Rust + Python混合架构
- **TTS引擎**: 自研混合方案（神经网络 + 参数合成）
- **模型格式**: .pth（模型权重）、.index（特征索引）、.emod（情绪控制）

### 1.3 系统特性

```
┌─────────────────────────────────────────────────────────────┐
│                      核心特性矩阵                              │
├─────────────────┬───────────────────────────────────────────┤
│ 高性能推理      │ Rust核心引擎，毫秒级响应                    │
│ 情绪可控        │ EMOD文件精确控制情感表达                    │
│ 自动化标注      │ Whisper ASR自动识别，人工可修正              │
│ 模块化设计      │ 前后端分离，组件可独立升级                   │
│ 原生技术栈      │ 无框架依赖，轻量高效                        │
└─────────────────┴───────────────────────────────────────────┘
```

---

## 2. 系统架构设计

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           用户界面层 (Frontend)                          │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │  模型管理   │ │  文本朗读   │ │  训练中心   │ │  标注工具   │       │
│  │   页面      │ │   页面      │ │   页面      │ │   页面      │       │
│  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘ └──────┬──────┘       │
│         │               │               │               │               │
│  ┌──────┴───────────────┴───────────────┴───────────────┴──────┐       │
│  │              原生JavaScript模块化路由系统                      │       │
│  └────────────────────────────┬────────────────────────────────┘       │
└───────────────────────────────┼─────────────────────────────────────────┘
                                │ HTTP/WebSocket
                                ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         服务网关层 (Rust Gateway)                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │  路由分发   │ │  认证鉴权   │ │  限流熔断   │ │  日志监控   │       │
│  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘ └──────┬──────┘       │
│         └───────────────┴───────────────┴───────────────┘               │
│                          Actix-web / Axum                               │
└───────────────────────────────┬─────────────────────────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐       ┌───────────────┐       ┌───────────────┐
│  TTS引擎服务   │       │  训练服务      │       │  标注服务      │
│   (Python)    │       │  (Python)     │       │  (Python)     │
├───────────────┤       ├───────────────┤       ├───────────────┤
│ • 文本分析    │       │ • 数据预处理   │       │ • ASR识别     │
│ • 声学模型    │       │ • 模型训练    │       │ • 文本对齐    │
│ • 声码器     │       │ • 特征提取    │       │ • 标注修正    │
│ • 情绪注入    │       │ • 模型导出    │       │ • 数据导出    │
└───────┬───────┘       └───────┬───────┘       └───────┬───────┘
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         数据存储层                                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │  模型存储   │ │  音频存储   │ │  标注数据   │ │  配置存储   │       │
│  │  (文件系统)  │ │  (文件系统)  │ │  (SQLite)   │ │  (JSON)     │       │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘       │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2 模块职责划分

#### 2.2.1 Rust核心模块

| 模块名称 | 职责 | 技术实现 |
|---------|------|---------|
| **api-gateway** | HTTP路由、请求分发、响应封装 | Actix-web / Axum |
| **auth-module** | 用户认证、权限验证、会话管理 | JWT + Redis |
| **file-manager** | 文件上传下载、存储管理 | Tokio + async-std |
| **task-scheduler** | 任务队列、训练调度、状态管理 | Tokio Channels |
| **cache-layer** | 热点数据缓存、模型预加载 | DashMap + LRU |
| **python-bridge** | Python模块调用、数据交换 | PyO3 |

#### 2.2.2 Python核心模块

| 模块名称 | 职责 | 技术实现 |
|---------|------|---------|
| **tts-engine** | 文本分析、声学建模、语音合成 | PyTorch + NumPy |
| **trainer** | 模型训练、特征提取、模型导出 | PyTorch Lightning |
| **annotator** | 音频识别、文本对齐、标注管理 | Whisper + forced-align |
| **feature-extractor** | 音频特征提取、声学特征计算 | librosa + soundfile |

### 2.3 技术栈总览

```
┌─────────────────────────────────────────────────────────────┐
│                        技术栈架构                            │
├─────────────────────────────────────────────────────────────┤
│  前端        │ HTML5 + CSS3 + ES6+ Modules + Web Audio API  │
├─────────────────────────────────────────────────────────────┤
│  网关层      │ Rust + Actix-web/Axum + Tokio + PyO3         │
├─────────────────────────────────────────────────────────────┤
│  AI服务层    │ Python 3.10+ + PyTorch + NumPy + librosa     │
├─────────────────────────────────────────────────────────────┤
│  数据层      │ SQLite + 文件系统 + JSON配置                  │
├─────────────────────────────────────────────────────────────┤
│  通信协议    │ HTTP/1.1 + WebSocket + JSON-RPC              │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 自制TTS引擎设计

### 3.1 TTS引擎架构

采用**混合方案**：神经网络前端 + 参数化声码器，兼顾音质与可控性。

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        自制TTS引擎架构                                   │
│                                                                         │
│  输入文本                                                                │
│     │                                                                   │
│     ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    文本前端处理模块                              │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │   │
│  │  │ 文本清洗 │→│ 分词处理 │→│ 音素转换 │→│ 韵律预测 │       │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘       │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│                                  ▼ 音素序列 + 韵律标签                   │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                  声学模型 (神经网络)                             │   │
│  │  ┌──────────────────────────────────────────────────────────┐  │   │
│  │  │              Emo-Tacotron 编码器-解码器架构                │  │   │
│  │  │                                                           │  │   │
│  │  │   ┌─────────┐    ┌─────────┐    ┌─────────┐             │  │   │
│  │  │   │ 编码器   │ →  │ 注意力   │ →  │ 解码器   │             │  │   │
│  │  │   │(Encoder) │    │(Attention)│    │(Decoder) │             │  │   │
│  │  │   └─────────┘    └─────────┘    └────┬────┘             │  │   │
│  │  │                                       │                   │  │   │
│  │  │   ┌───────────────────────────────────┘                   │  │   │
│  │  │   │                                                       │  │   │
│  │  │   ▼                                                       │  │   │
│  │  │  ┌──────────────────────────────────────────────┐        │  │   │
│  │  │  │           情绪嵌入层 (Emotion Embedding)       │        │  │   │
│  │  │  │         ← 输入: EMOD情绪向量                   │        │  │   │
│  │  │  └──────────────────────────────────────────────┘        │  │   │
│  │  └──────────────────────────────────────────────────────────┘  │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│                                  ▼ 梅尔频谱图                            │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                  声码器 (参数化 + 神经混合)                       │   │
│  │  ┌──────────────────────────────────────────────────────────┐  │   │
│  │  │                  HiFi-GAN / WaveGlow                      │  │   │
│  │  │                                                           │  │   │
│  │  │   梅尔频谱 → 上采样网络 → 多尺度生成 → 波形输出            │  │   │
│  │  │                                                           │  │   │
│  │  │   ┌─────────────────────────────────────────────────┐    │  │   │
│  │  │   │  参数化控制层 (基频、时长、能量微调)              │    │  │   │
│  │  │   └─────────────────────────────────────────────────┘    │  │   │
│  │  └──────────────────────────────────────────────────────────┘  │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│                                  ▼                                      │
│                            音频波形输出                                  │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 核心组件设计

#### 3.2.1 文本前端处理器

```python
# tts_engine/frontend/text_processor.py

class TextProcessor:
    """文本前端处理模块"""
    
    def __init__(self, language: str = 'zh'):
        self.language = language
        self.tokenizer = TextTokenizer(language)
        self.phonemizer = Phonemizer(language)
        self.prosody_predictor = ProsodyPredictor()
    
    def process(self, text: str) -> dict:
        """
        文本处理流程
        
        Args:
            text: 输入文本
            
        Returns:
            {
                'phonemes': 音素序列,
                'durations': 预测时长,
                'pitch_contour': 基频轮廓,
                'prosody_tags': 韵律标签
            }
        """
        # 1. 文本清洗
        cleaned_text = self._clean_text(text)
        
        # 2. 分词
        words = self.tokenizer.tokenize(cleaned_text)
        
        # 3. 音素转换
        phonemes = self.phonemizer.phonemize(words)
        
        # 4. 韵律预测
        prosody = self.prosody_predictor.predict(phonemes)
        
        return {
            'phonemes': phonemes,
            'durations': prosody['durations'],
            'pitch_contour': prosody['pitch'],
            'prosody_tags': prosody['tags']
        }
```

#### 3.2.2 声学模型 (Emo-Tacotron)

```python
# tts_engine/acoustic/emo_tacotron.py

import torch
import torch.nn as nn

class EmotionEmbedding(nn.Module):
    """情绪嵌入层 - 核心创新点"""
    
    def __init__(self, emotion_dim: int = 256, hidden_dim: int = 512):
        super().__init__()
        self.emotion_proj = nn.Sequential(
            nn.Linear(emotion_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, hidden_dim),
            nn.Tanh()
        )
        
    def forward(self, emotion_vector: torch.Tensor) -> torch.Tensor:
        """
        将EMOD情绪向量转换为可融合的嵌入
        
        Args:
            emotion_vector: 来自EMOD文件的情绪向量 [batch, emotion_dim]
            
        Returns:
            情绪嵌入 [batch, hidden_dim]
        """
        return self.emotion_proj(emotion_vector)


class EmoTacotron(nn.Module):
    """带情绪控制的Tacotron声学模型"""
    
    def __init__(self, 
                 vocab_size: int,
                 emotion_dim: int = 256,
                 mel_dim: int = 80,
                 hidden_dim: int = 512):
        super().__init__()
        
        # 编码器
        self.encoder = TextEncoder(vocab_size, hidden_dim)
        
        # 情绪嵌入
        self.emotion_embedding = EmotionEmbedding(emotion_dim, hidden_dim)
        
        # 注意力机制
        self.attention = LocationSensitiveAttention(hidden_dim)
        
        # 解码器
        self.decoder = DecoderRNN(hidden_dim, mel_dim)
        
        # 后处理网络
        self.postnet = PostNet(mel_dim)
        
    def forward(self, 
                text_inputs: torch.Tensor,
                emotion_vector: torch.Tensor,
                mel_targets: torch.Tensor = None):
        """
        前向传播
        
        Args:
            text_inputs: 文本编码 [batch, text_len]
            emotion_vector: 情绪向量 [batch, emotion_dim]
            mel_targets: 目标梅尔频谱(训练时使用)
            
        Returns:
            mel_outputs: 预测的梅尔频谱
            attention_weights: 注意力权重
        """
        # 编码文本
        encoder_outputs = self.encoder(text_inputs)
        
        # 获取情绪嵌入
        emotion_embed = self.emotion_embedding(emotion_vector)
        
        # 融合情绪信息到编码器输出
        emotion_expanded = emotion_embed.unsqueeze(1).expand(-1, encoder_outputs.size(1), -1)
        encoder_outputs = encoder_outputs + emotion_expanded
        
        # 解码生成梅尔频谱
        mel_outputs, attention_weights = self.decoder(
            encoder_outputs, 
            self.attention,
            mel_targets
        )
        
        # 后处理优化
        mel_outputs = self.postnet(mel_outputs)
        
        return mel_outputs, attention_weights
```

#### 3.2.3 声码器 (HiFi-GAN + 参数控制)

```python
# tts_engine/vocoder/hifigan.py

import torch
import torch.nn as nn

class HiFiGANVocoder(nn.Module):
    """HiFi-GAN声码器，支持参数化控制"""
    
    def __init__(self, 
                 mel_dim: int = 80,
                 upsample_rates: list = [8, 8, 2, 2]):
        super().__init__()
        
        # 上采样网络
        self.upsample_net = nn.ModuleList([
            UpsampleBlock(512 // (2 ** i), upsample_rates[i])
            for i in range(len(upsample_rates))
        ])
        
        # 多尺度残差块
        self.res_blocks = nn.ModuleList([
            ResBlock(256, [3, 7, 11])
            for _ in range(3)
        ])
        
        # 参数控制层
        self.param_controller = ParameterController()
        
    def forward(self, 
                mel: torch.Tensor,
                f0: torch.Tensor = None,
                energy: torch.Tensor = None) -> torch.Tensor:
        """
        梅尔频谱转波形
        
        Args:
            mel: 梅尔频谱 [batch, mel_dim, time]
            f0: 基频曲线(可选)
            energy: 能量曲线(可选)
            
        Returns:
            waveform: 音频波形 [batch, 1, samples]
        """
        # 上采样
        x = mel
        for upsample in self.upsample_net:
            x = upsample(x)
        
        # 多尺度残差处理
        for res_block in self.res_blocks:
            x = res_block(x)
        
        # 参数化微调
        if f0 is not None or energy is not None:
            x = self.param_controller(x, f0, energy)
        
        # 输出波形
        waveform = torch.tanh(x)
        
        return waveform


class ParameterController(nn.Module):
    """参数化控制层 - 用于精细调节"""
    
    def __init__(self):
        super().__init__()
        self.f0_modulator = F0Modulator()
        self.energy_modulator = EnergyModulator()
        
    def forward(self, x, f0=None, energy=None):
        if f0 is not None:
            x = self.f0_modulator(x, f0)
        if energy is not None:
            x = self.energy_modulator(x, energy)
        return x
```

### 3.3 模型文件结构

```
models/
├── voice_model_001/
│   ├── model.pth              # PyTorch模型权重文件
│   ├── config.json            # 模型配置参数
│   ├── vocab.json             # 词表/音素表
│   ├── features.index         # 特征索引文件
│   └── emotion.emod           # 情绪控制文件
│
├── vocoder/
│   ├── hifigan.pth           # 声码器权重
│   └── config.json           # 声码器配置
│
└── pretrained/
    ├── encoder.pth           # 预训练编码器
    └── aligner.pth           # 预训练对齐器
```

---

## 4. 情绪控制文件(EMOD)设计

### 4.1 EMOD文件格式定义

EMOD (Emotion Descriptor) 是本项目独创的情绪控制文件格式，采用JSON结构，支持多维度的情绪参数化控制。

```json
{
    "version": "1.0",
    "metadata": {
        "name": "温暖亲切",
        "description": "适合讲故事、朗读散文的温暖音色",
        "author": "user_001",
        "created_at": "2025-05-10T10:30:00Z",
        "tags": ["warm", "friendly", "storytelling"]
    },
    
    "emotion_vector": {
        "dimensions": {
            "valence": 0.75,        // 愉悦度 [-1, 1]，正值表示积极
            "arousal": 0.35,        // 唤醒度 [-1, 1]，正值表示活跃
            "dominance": 0.20       // 支配度 [-1, 1]，正值表示自信
        },
        "custom_dims": {
            "warmth": 0.80,         // 温暖度
            "sincerity": 0.70       // 真诚度
        }
    },
    
    "prosody_control": {
        "pitch": {
            "base_offset": 2.0,     // 基频偏移 (半音)
            "range_scale": 1.2,     // 音高范围缩放
            "variation": 0.6        // 变化程度
        },
        "duration": {
            "speaking_rate": 0.9,   // 语速系数 (1.0为正常)
            "pause_duration": 1.1,  // 停顿时长系数
            "emphasis_factor": 1.3  // 重音强调系数
        },
        "energy": {
            "base_level": 0.7,      // 基础能量级别
            "variation": 0.5        // 能量变化程度
        }
    },
    
    "style_markers": [
        {
            "type": "sentence_start",
            "pitch_curve": "rise",
            "duration_factor": 1.1
        },
        {
            "type": "sentence_end",
            "pitch_curve": "fall",
            "duration_factor": 1.2
        },
        {
            "type": "question",
            "pitch_curve": "rise_sharp",
            "duration_factor": 1.0
        }
    ],
    
    "voice_characteristics": {
        "breathiness": 0.2,         // 气声程度
        "roughness": 0.1,           // 粗糙度
        "creakiness": 0.05,         // 嘎裂声程度
        "formant_shift": 0.0        // 共振峰偏移
    }
}
```

### 4.2 EMOD文件解析器

```python
# tts_engine/emotion/emod_parser.py

import json
import numpy as np
from dataclasses import dataclass
from typing import Dict, List, Optional

@dataclass
class EmotionVector:
    """情绪向量数据结构"""
    valence: float      # 愉悦度
    arousal: float      # 唤醒度
    dominance: float    # 支配度
    custom: Dict[str, float]  # 自定义维度

@dataclass
class ProsodyControl:
    """韵律控制参数"""
    pitch_offset: float
    pitch_range: float
    pitch_variation: float
    speaking_rate: float
    pause_duration: float
    energy_level: float

class EmodParser:
    """EMOD文件解析器"""
    
    def __init__(self, emod_path: str):
        self.emod_path = emod_path
        self.data = self._load_emod()
        
    def _load_emod(self) -> dict:
        """加载EMOD文件"""
        with open(self.emod_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    
    def get_emotion_vector(self) -> np.ndarray:
        """
        获取情绪向量，用于模型输入
        
        Returns:
            emotion_vector: [valence, arousal, dominance, ...custom_dims]
        """
        dims = self.data['emotion_vector']['dimensions']
        custom = self.data['emotion_vector'].get('custom_dims', {})
        
        vector = [
            dims['valence'],
            dims['arousal'],
            dims['dominance']
        ]
        
        # 添加自定义维度
        for key in sorted(custom.keys()):
            vector.append(custom[key])
            
        return np.array(vector, dtype=np.float32)
    
    def get_prosody_control(self) -> ProsodyControl:
        """获取韵律控制参数"""
        prosody = self.data['prosody_control']
        return ProsodyControl(
            pitch_offset=prosody['pitch']['base_offset'],
            pitch_range=prosody['pitch']['range_scale'],
            pitch_variation=prosody['pitch']['variation'],
            speaking_rate=prosody['duration']['speaking_rate'],
            pause_duration=prosody['duration']['pause_duration'],
            energy_level=prosody['energy']['base_level']
        )
    
    def get_style_markers(self) -> List[dict]:
        """获取风格标记"""
        return self.data.get('style_markers', [])
    
    def get_voice_characteristics(self) -> Dict[str, float]:
        """获取声音特征"""
        return self.data.get('voice_characteristics', {})


class EmodGenerator:
    """EMOD文件生成器"""
    
    @staticmethod
    def create_from_params(
        name: str,
        valence: float = 0.0,
        arousal: float = 0.0,
        dominance: float = 0.0,
        custom_dims: Dict[str, float] = None,
        prosody: Dict = None,
        voice_chars: Dict = None
    ) -> dict:
        """
        从参数创建EMOD数据结构
        
        Args:
            name: 情绪名称
            valence: 愉悦度
            arousal: 唤醒度
            dominance: 支配度
            custom_dims: 自定义维度
            prosody: 韵律参数
            voice_chars: 声音特征
            
        Returns:
            EMOD数据结构
        """
        emod_data = {
            "version": "1.0",
            "metadata": {
                "name": name,
                "description": f"Auto-generated emotion: {name}",
                "created_at": datetime.now().isoformat()
            },
            "emotion_vector": {
                "dimensions": {
                    "valence": valence,
                    "arousal": arousal,
                    "dominance": dominance
                },
                "custom_dims": custom_dims or {}
            },
            "prosody_control": prosody or EmodGenerator._default_prosody(),
            "voice_characteristics": voice_chars or EmodGenerator._default_voice()
        }
        return emod_data
    
    @staticmethod
    def _default_prosody() -> dict:
        return {
            "pitch": {"base_offset": 0.0, "range_scale": 1.0, "variation": 0.5},
            "duration": {"speaking_rate": 1.0, "pause_duration": 1.0},
            "energy": {"base_level": 0.6, "variation": 0.4}
        }
    
    @staticmethod
    def _default_voice() -> dict:
        return {
            "breathiness": 0.1,
            "roughness": 0.1,
            "creakiness": 0.05,
            "formant_shift": 0.0
        }
    
    @staticmethod
    def save(emod_data: dict, output_path: str):
        """保存EMOD文件"""
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(emod_data, f, ensure_ascii=False, indent=2)
```

### 4.3 预设情绪模板

```python
# tts_engine/emotion/emotion_presets.py

EMOTION_PRESETS = {
    "neutral": {
        "valence": 0.0, "arousal": 0.0, "dominance": 0.0,
        "description": "中性、平静的朗读风格"
    },
    "happy": {
        "valence": 0.8, "arousal": 0.6, "dominance": 0.3,
        "pitch_offset": 3.0, "speaking_rate": 1.1,
        "description": "快乐、积极的情绪"
    },
    "sad": {
        "valence": -0.7, "arousal": -0.3, "dominance": -0.4,
        "pitch_offset": -2.0, "speaking_rate": 0.85,
        "description": "悲伤、低沉的情绪"
    },
    "angry": {
        "valence": -0.5, "arousal": 0.8, "dominance": 0.6,
        "pitch_offset": 4.0, "speaking_rate": 1.2,
        "description": "愤怒、激动的情绪"
    },
    "calm": {
        "valence": 0.3, "arousal": -0.4, "dominance": 0.1,
        "pitch_offset": -1.0, "speaking_rate": 0.9,
        "description": "平静、放松的情绪"
    },
    "excited": {
        "valence": 0.7, "arousal": 0.9, "dominance": 0.5,
        "pitch_offset": 5.0, "speaking_rate": 1.15,
        "description": "兴奋、热情的情绪"
    },
    "storytelling": {
        "valence": 0.4, "arousal": 0.2, "dominance": 0.3,
        "pitch_variation": 0.8, "pause_duration": 1.3,
        "description": "讲故事风格，富有表现力"
    }
}
```

---

## 5. 自动音频标注系统设计

### 5.1 标注系统架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        自动音频标注系统架构                               │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        输入层                                    │   │
│  │   音频文件 (wav/mp3/flac) → 音频预处理 → 分段切割               │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│                                  ▼                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    ASR识别模块 (Whisper)                         │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │   │
│  │  │ 特征提取 │→│ 编码器   │→│ 解码器   │→│ 文本输出 │       │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘       │   │
│  │                                                                  │   │
│  │  输出: 转录文本 + 时间戳                                         │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│                                  ▼                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    强制对齐模块 (Montreal Forced Aligner)        │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐                       │   │
│  │  │ 音素转换 │→│ HMM对齐  │→│ 时间校正 │                       │   │
│  │  └──────────┘  └──────────┘  └──────────┘                       │   │
│  │                                                                  │   │
│  │  输出: 音素级别时间对齐                                          │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│                                  ▼                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    标注编辑界面 (Web)                            │   │
│  │  ┌──────────────────────────────────────────────────────────┐  │   │
│  │  │  波形显示  │  文本编辑  │  时间轴调整  │  验证标记       │  │   │
│  │  └──────────────────────────────────────────────────────────┘  │   │
│  │                                                                  │   │
│  │  功能: 可视化编辑、手动修正、批量操作                            │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│                                  ▼                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    标注数据导出                                  │   │
│  │   格式: JSON / CSV / Label Studio格式 / 自定义格式              │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.2 ASR自动识别模块

```python
# annotator/asr/whisper_recognizer.py

import whisper
import torch
from typing import List, Dict
import librosa

class WhisperASR:
    """基于Whisper的自动语音识别"""
    
    def __init__(self, model_size: str = "medium", device: str = "cuda"):
        """
        初始化Whisper模型
        
        Args:
            model_size: 模型大小 (tiny/base/small/medium/large)
            device: 计算设备 (cuda/cpu)
        """
        self.model = whisper.load_model(model_size, device=device)
        self.device = device
        
    def transcribe(self, 
                   audio_path: str,
                   language: str = "zh",
                   word_timestamps: bool = True) -> Dict:
        """
        转录音频文件
        
        Args:
            audio_path: 音频文件路径
            language: 语言代码
            word_timestamps: 是否输出词级时间戳
            
        Returns:
            {
                'text': 完整文本,
                'segments': [
                    {
                        'start': 开始时间,
                        'end': 结束时间,
                        'text': 片段文本,
                        'words': [词级时间戳]
                    }
                ]
            }
        """
        # 加载音频
        audio = whisper.load_audio(audio_path)
        
        # 转录
        result = self.model.transcribe(
            audio,
            language=language,
            word_timestamps=word_timestamps,
            fp16=torch.cuda.is_available()
        )
        
        return self._format_result(result)
    
    def _format_result(self, raw_result: dict) -> Dict:
        """格式化输出结果"""
        formatted = {
            'text': raw_result['text'].strip(),
            'segments': []
        }
        
        for segment in raw_result.get('segments', []):
            seg_data = {
                'start': round(segment['start'], 3),
                'end': round(segment['end'], 3),
                'text': segment['text'].strip(),
                'confidence': segment.get('avg_logprob', 0),
                'words': []
            }
            
            # 词级时间戳
            if 'words' in segment:
                for word in segment['words']:
                    seg_data['words'].append({
                        'word': word['word'],
                        'start': round(word['start'], 3),
                        'end': round(word['end'], 3),
                        'probability': word.get('probability', 1.0)
                    })
                    
            formatted['segments'].append(seg_data)
            
        return formatted


class BatchTranscriber:
    """批量转录处理器"""
    
    def __init__(self, asr_model: WhisperASR):
        self.asr = asr_model
        
    def process_directory(self, 
                          input_dir: str,
                          output_file: str,
                          language: str = "zh") -> List[Dict]:
        """
        批量处理目录中的音频文件
        
        Args:
            input_dir: 输入目录
            output_file: 输出标注文件路径
            language: 语言代码
            
        Returns:
            标注结果列表
        """
        import os
        from pathlib import Path
        
        results = []
        audio_extensions = ['.wav', '.mp3', '.flac', '.ogg']
        
        for file_path in Path(input_dir).iterdir():
            if file_path.suffix.lower() in audio_extensions:
                print(f"Processing: {file_path.name}")
                
                transcription = self.asr.transcribe(
                    str(file_path),
                    language=language
                )
                
                results.append({
                    'audio_file': file_path.name,
                    'audio_path': str(file_path),
                    'transcription': transcription
                })
        
        # 保存结果
        self._save_results(results, output_file)
        
        return results
    
    def _save_results(self, results: List[Dict], output_path: str):
        """保存标注结果"""
        import json
        
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(results, f, ensure_ascii=False, indent=2)
```

### 5.3 标注编辑器后端

```python
# annotator/editor/annotation_manager.py

import sqlite3
import json
from typing import List, Dict, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class Annotation:
    """标注数据结构"""
    id: int
    audio_file: str
    text: str
    start_time: float
    end_time: float
    verified: bool
    created_at: str
    updated_at: str

class AnnotationManager:
    """标注数据管理器"""
    
    def __init__(self, db_path: str = "annotations.db"):
        self.db_path = db_path
        self._init_database()
        
    def _init_database(self):
        """初始化数据库"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS annotations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                audio_file TEXT NOT NULL,
                text TEXT NOT NULL,
                start_time REAL,
                end_time REAL,
                verified BOOLEAN DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS word_alignments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                annotation_id INTEGER,
                word TEXT NOT NULL,
                start_time REAL,
                end_time REAL,
                confidence REAL,
                FOREIGN KEY (annotation_id) REFERENCES annotations(id)
            )
        ''')
        
        conn.commit()
        conn.close()
        
    def add_annotation(self, 
                       audio_file: str,
                       text: str,
                       start_time: float = None,
                       end_time: float = None) -> int:
        """添加新标注"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            INSERT INTO annotations (audio_file, text, start_time, end_time)
            VALUES (?, ?, ?, ?)
        ''', (audio_file, text, start_time, end_time))
        
        annotation_id = cursor.lastrowid
        conn.commit()
        conn.close()
        
        return annotation_id
    
    def update_annotation(self, 
                          annotation_id: int,
                          text: str = None,
                          start_time: float = None,
                          end_time: float = None,
                          verified: bool = None) -> bool:
        """更新标注"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        updates = []
        params = []
        
        if text is not None:
            updates.append("text = ?")
            params.append(text)
        if start_time is not None:
            updates.append("start_time = ?")
            params.append(start_time)
        if end_time is not None:
            updates.append("end_time = ?")
            params.append(end_time)
        if verified is not None:
            updates.append("verified = ?")
            params.append(verified)
            
        updates.append("updated_at = CURRENT_TIMESTAMP")
        params.append(annotation_id)
        
        query = f"UPDATE annotations SET {', '.join(updates)} WHERE id = ?"
        cursor.execute(query, params)
        
        success = cursor.rowcount > 0
        conn.commit()
        conn.close()
        
        return success
    
    def get_annotations(self, 
                        audio_file: str = None,
                        verified: bool = None) -> List[Annotation]:
        """获取标注列表"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        query = "SELECT * FROM annotations WHERE 1=1"
        params = []
        
        if audio_file:
            query += " AND audio_file = ?"
            params.append(audio_file)
        if verified is not None:
            query += " AND verified = ?"
            params.append(verified)
            
        cursor.execute(query, params)
        rows = cursor.fetchall()
        conn.close()
        
        return [Annotation(*row) for row in rows]
    
    def export_for_training(self, output_path: str):
        """导出训练数据格式"""
        annotations = self.get_annotations()
        
        training_data = []
        for ann in annotations:
            if ann.verified:  # 只导出已验证的标注
                training_data.append({
                    'audio_filepath': ann.audio_file,
                    'text': ann.text,
                    'duration': ann.end_time - ann.start_time if ann.end_time and ann.start_time else None
                })
        
        with open(output_path, 'w', encoding='utf-8') as f:
            for item in training_data:
                f.write(json.dumps(item, ensure_ascii=False) + '\n')
```

### 5.4 标注工作流程

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        标注工作流程                                      │
│                                                                         │
│  1. 音频上传                                                            │
│     └── 用户上传训练音频文件                                            │
│                                                                         │
│  2. 自动转录                                                            │
│     ├── Whisper ASR自动识别文本                                         │
│     ├── 生成词级时间戳                                                  │
│     └── 存入标注数据库                                                  │
│                                                                         │
│  3. 人工审核                                                            │
│     ├── 可视化波形显示                                                  │
│     ├── 文本编辑修正                                                    │
│     ├── 时间边界调整                                                    │
│     └── 标记验证状态                                                    │
│                                                                         │
│  4. 数据导出                                                            │
│     ├── 导出训练格式 (JSONL)                                            │
│     ├── 导出特征索引 (.index)                                           │
│     └── 准备训练数据集                                                  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 6. 前端架构设计

### 6.1 前端技术架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        前端架构 (原生技术栈)                             │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    入口层 (index.html)                          │   │
│  │  ┌──────────────────────────────────────────────────────────┐  │   │
│  │  │  - 应用容器挂载点                                         │  │   │
│  │  │  - 全局样式引入                                           │  │   │
│  │  │  - 模块加载器配置                                         │  │   │
│  │  └──────────────────────────────────────────────────────────┘  │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│  ┌───────────────────────────────┴─────────────────────────────────┐   │
│  │                    核心模块层 (ES6 Modules)                      │   │
│  │                                                                  │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ Router   │ │ State    │ │ EventBus │ │ Http     │          │   │
│  │  │ 路由管理  │ │ 状态管理  │ │ 事件总线  │ │ HTTP客户端│          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│  ┌───────────────────────────────┴─────────────────────────────────┐   │
│  │                    组件层 (Web Components)                       │   │
│  │                                                                  │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ 音频播放器│ │ 波形显示  │ │ 文本编辑器│ │ 情绪面板 │          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  │                                                                  │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ 模型卡片 │ │ 进度条   │ │ 文件上传  │ │ 参数滑块 │          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│  ┌───────────────────────────────┴─────────────────────────────────┐   │
│  │                    页面层 (Page Modules)                         │   │
│  │                                                                  │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ 首页     │ │ 模型管理  │ │ 文本朗读  │ │ 训练中心 │          │   │
│  │  │ HomePage │ │ Models   │ │ TTS      │ │ Training │          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  │                                                                  │   │
│  │  ┌──────────┐ ┌──────────┐                                      │   │
│  │  │ 标注工具  │ │ 设置页面  │                                      │   │
│  │  │ Annotate │ │ Settings │                                      │   │
│  │  └──────────┘ └──────────┘                                      │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    工具层 (Utilities)                            │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ 音频处理 │ │ 文件工具  │ │ 格式化   │ │ 验证器   │          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

### 6.2 目录结构

```
frontend/
├── index.html                    # 应用入口
├── styles/
│   ├── main.css                 # 全局样式
│   ├── components.css           # 组件样式
│   ├── pages.css                # 页面样式
│   └── themes/
│       ├── light.css            # 浅色主题
│       └── dark.css             # 深色主题
│
├── scripts/
│   ├── app.js                   # 应用主入口
│   ├── core/
│   │   ├── router.js            # 路由管理器
│   │   ├── state.js             # 状态管理器
│   │   ├── eventBus.js          # 事件总线
│   │   └── http.js              # HTTP客户端
│   │
│   ├── components/
│   │   ├── AudioPlayer.js       # 音频播放器组件
│   │   ├── WaveformViewer.js    # 波形显示组件
│   │   ├── TextEditor.js        # 文本编辑器组件
│   │   ├── EmotionPanel.js      # 情绪控制面板
│   │   ├── ModelCard.js         # 模型卡片组件
│   │   ├── FileUpload.js        # 文件上传组件
│   │   └── Slider.js            # 参数滑块组件
│   │
│   ├── pages/
│   │   ├── HomePage.js          # 首页
│   │   ├── ModelsPage.js        # 模型管理页
│   │   ├── TTSPage.js           # 文本朗读页
│   │   ├── TrainingPage.js      # 训练中心页
│   │   ├── AnnotatePage.js      # 标注工具页
│   │   └── SettingsPage.js      # 设置页面
│   │
│   └── utils/
│       ├── audioUtils.js        # 音频处理工具
│       ├── fileUtils.js         # 文件操作工具
│       ├── formatters.js        # 格式化工具
│       └── validators.js        # 验证工具
│
└── assets/
    ├── icons/                   # 图标资源
    ├── images/                  # 图片资源
    └── fonts/                   # 字体资源
```

### 6.3 核心模块实现

#### 6.3.1 路由管理器

```javascript
// scripts/core/router.js

export class Router {
    constructor() {
        this.routes = new Map();
        this.currentRoute = null;
        this.params = {};
        
        // 监听hash变化
        window.addEventListener('hashchange', () => this.handleRouteChange());
        window.addEventListener('load', () => this.handleRouteChange());
    }
    
    /**
     * 注册路由
     * @param {string} path - 路由路径
     * @param {Function} handler - 处理函数
     */
    register(path, handler) {
        this.routes.set(path, handler);
    }
    
    /**
     * 导航到指定路由
     * @param {string} path - 目标路径
     * @param {Object} params - 路由参数
     */
    navigate(path, params = {}) {
        this.params = params;
        window.location.hash = path;
    }
    
    /**
     * 处理路由变化
     */
    async handleRouteChange() {
        const hash = window.location.hash.slice(1) || '/';
        const [path, queryString] = hash.split('?');
        
        // 解析查询参数
        if (queryString) {
            const searchParams = new URLSearchParams(queryString);
            searchParams.forEach((value, key) => {
                this.params[key] = value;
            });
        }
        
        // 查找匹配的路由
        const handler = this.routes.get(path);
        if (handler) {
            this.currentRoute = path;
            await handler(this.params);
        } else {
            // 404处理
            const notFoundHandler = this.routes.get('/404');
            if (notFoundHandler) {
                await notFoundHandler();
            }
        }
    }
    
    /**
     * 获取当前路由参数
     */
    getParams() {
        return { ...this.params };
    }
}

// 创建全局路由实例
export const router = new Router();
```

#### 6.3.2 状态管理器

```javascript
// scripts/core/state.js

export class StateManager {
    constructor(initialState = {}) {
        this.state = initialState;
        this.listeners = new Map();
        this.history = [];
        this.maxHistoryLength = 50;
    }
    
    /**
     * 获取状态值
     * @param {string} key - 状态键名
     */
    get(key) {
        return key ? this.state[key] : { ...this.state };
    }
    
    /**
     * 设置状态值
     * @param {string|Object} key - 状态键名或状态对象
     * @param {*} value - 状态值
     */
    set(key, value) {
        const oldState = { ...this.state };
        
        if (typeof key === 'object') {
            this.state = { ...this.state, ...key };
        } else {
            this.state[key] = value;
        }
        
        // 记录历史
        this.history.push({ oldState, newState: { ...this.state } });
        if (this.history.length > this.maxHistoryLength) {
            this.history.shift();
        }
        
        // 触发监听器
        this.notify(key, value, oldState);
    }
    
    /**
     * 订阅状态变化
     * @param {string} key - 状态键名
     * @param {Function} callback - 回调函数
     */
    subscribe(key, callback) {
        if (!this.listeners.has(key)) {
            this.listeners.set(key, new Set());
        }
        this.listeners.get(key).add(callback);
        
        // 返回取消订阅函数
        return () => {
            this.listeners.get(key).delete(callback);
        };
    }
    
    /**
     * 通知状态变化
     */
    notify(key, newValue, oldState) {
        const listeners = this.listeners.get(key);
        if (listeners) {
            listeners.forEach(callback => {
                callback(newValue, oldState[key]);
            });
        }
        
        // 通知全局监听器
        const globalListeners = this.listeners.get('*');
        if (globalListeners) {
            globalListeners.forEach(callback => {
                callback(this.state, oldState);
            });
        }
    }
    
    /**
     * 重置状态
     */
    reset(initialState = {}) {
        const oldState = { ...this.state };
        this.state = initialState;
        this.notify('*', this.state, oldState);
    }
}

// 创建全局状态实例
export const store = new StateManager({
    currentModel: null,
    models: [],
    isPlaying: false,
    currentAudio: null,
    emotionSettings: {},
    trainingStatus: null
});
```

#### 6.3.3 音频播放器组件

```javascript
// scripts/components/AudioPlayer.js

export class AudioPlayer extends HTMLElement {
    constructor() {
        super();
        this.audio = null;
        this.isPlaying = false;
        this.currentTime = 0;
        this.duration = 0;
        this.volume = 1.0;
        
        this.attachShadow({ mode: 'open' });
        this.render();
    }
    
    connectedCallback() {
        this.audio = new Audio();
        this.setupEventListeners();
    }
    
    disconnectedCallback() {
        this.audio.pause();
        this.audio.src = '';
    }
    
    /**
     * 加载音频文件
     * @param {string} src - 音频URL或Blob
     */
    load(src) {
        return new Promise((resolve, reject) => {
            this.audio.src = src;
            this.audio.onloadedmetadata = () => {
                this.duration = this.audio.duration;
                this.updateUI();
                resolve();
            };
            this.audio.onerror = reject;
        });
    }
    
    /**
     * 播放
     */
    async play() {
        try {
            await this.audio.play();
            this.isPlaying = true;
            this.updatePlayButton();
        } catch (error) {
            console.error('播放失败:', error);
        }
    }
    
    /**
     * 暂停
     */
    pause() {
        this.audio.pause();
        this.isPlaying = false;
        this.updatePlayButton();
    }
    
    /**
     * 停止
     */
    stop() {
        this.audio.pause();
        this.audio.currentTime = 0;
        this.isPlaying = false;
        this.updatePlayButton();
    }
    
    /**
     * 跳转到指定时间
     * @param {number} time - 时间(秒)
     */
    seek(time) {
        this.audio.currentTime = Math.max(0, Math.min(time, this.duration));
    }
    
    /**
     * 设置音量
     * @param {number} volume - 音量(0-1)
     */
    setVolume(volume) {
        this.volume = Math.max(0, Math.min(1, volume));
        this.audio.volume = this.volume;
        this.updateVolumeUI();
    }
    
    setupEventListeners() {
        this.audio.addEventListener('timeupdate', () => {
            this.currentTime = this.audio.currentTime;
            this.updateProgress();
        });
        
        this.audio.addEventListener('ended', () => {
            this.isPlaying = false;
            this.updatePlayButton();
            this.dispatchEvent(new CustomEvent('ended'));
        });
        
        // 播放按钮点击
        this.shadowRoot.querySelector('.play-btn').addEventListener('click', () => {
            this.isPlaying ? this.pause() : this.play();
        });
        
        // 进度条拖动
        this.shadowRoot.querySelector('.progress-bar').addEventListener('input', (e) => {
            this.seek(e.target.value * this.duration / 100);
        });
        
        // 音量控制
        this.shadowRoot.querySelector('.volume-slider').addEventListener('input', (e) => {
            this.setVolume(e.target.value / 100);
        });
    }
    
    render() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: flex;
                    flex-direction: column;
                    gap: 12px;
                    padding: 16px;
                    background: var(--card-bg, #f5f5f5);
                    border-radius: 8px;
                }
                
                .controls {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                }
                
                .play-btn {
                    width: 48px;
                    height: 48px;
                    border-radius: 50%;
                    border: none;
                    background: var(--primary-color, #4a90d9);
                    color: white;
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                
                .play-btn:hover {
                    background: var(--primary-hover, #3a7bc8);
                }
                
                .progress-container {
                    flex: 1;
                    display: flex;
                    align-items: center;
                    gap: 8px;
                }
                
                .progress-bar {
                    flex: 1;
                    height: 6px;
                    -webkit-appearance: none;
                    background: var(--progress-bg, #ddd);
                    border-radius: 3px;
                    cursor: pointer;
                }
                
                .time-display {
                    font-size: 12px;
                    color: var(--text-secondary, #666);
                    min-width: 80px;
                    text-align: center;
                }
                
                .volume-control {
                    display: flex;
                    align-items: center;
                    gap: 8px;
                }
                
                .volume-slider {
                    width: 80px;
                    height: 4px;
                    -webkit-appearance: none;
                    background: var(--volume-bg, #ddd);
                    border-radius: 2px;
                }
            </style>
            
            <div class="controls">
                <button class="play-btn">
                    <svg class="play-icon" viewBox="0 0 24 24" width="24" height="24">
                        <path fill="currentColor" d="M8 5v14l11-7z"/>
                    </svg>
                </button>
                
                <div class="progress-container">
                    <span class="time-display current-time">0:00</span>
                    <input type="range" class="progress-bar" min="0" max="100" value="0">
                    <span class="time-display duration">0:00</span>
                </div>
                
                <div class="volume-control">
                    <svg viewBox="0 0 24 24" width="20" height="20">
                        <path fill="currentColor" d="M3 9v6h4l5 5V4L7 9H3z"/>
                    </svg>
                    <input type="range" class="volume-slider" min="0" max="100" value="100">
                </div>
            </div>
        `;
    }
    
    updateUI() {
        this.updateProgress();
        this.updateTimeDisplay();
    }
    
    updateProgress() {
        const progress = (this.currentTime / this.duration) * 100 || 0;
        this.shadowRoot.querySelector('.progress-bar').value = progress;
        this.updateTimeDisplay();
    }
    
    updateTimeDisplay() {
        const formatTime = (seconds) => {
            const mins = Math.floor(seconds / 60);
            const secs = Math.floor(seconds % 60);
            return `${mins}:${secs.toString().padStart(2, '0')}`;
        };
        
        this.shadowRoot.querySelector('.current-time').textContent = formatTime(this.currentTime);
        this.shadowRoot.querySelector('.duration').textContent = formatTime(this.duration);
    }
    
    updatePlayButton() {
        const icon = this.shadowRoot.querySelector('.play-icon');
        if (this.isPlaying) {
            icon.innerHTML = '<path fill="currentColor" d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>';
        } else {
            icon.innerHTML = '<path fill="currentColor" d="M8 5v14l11-7z"/>';
        }
    }
    
    updateVolumeUI() {
        this.shadowRoot.querySelector('.volume-slider').value = this.volume * 100;
    }
}

// 注册Web Component
customElements.define('audio-player', AudioPlayer);
```

### 6.4 页面布局设计

#### 6.4.1 文本朗读页面

```
┌─────────────────────────────────────────────────────────────────────────┐
│  [Logo] AI朗读软件                              [模型选择 ▼] [设置 ⚙️]  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        文本输入区域                              │   │
│  │  ┌──────────────────────────────────────────────────────────┐  │   │
│  │  │  请输入要朗读的文本...                                    │  │   │
│  │  │                                                          │  │   │
│  │  │  支持多行输入，最多10000字符                              │  │   │
│  │  └──────────────────────────────────────────────────────────┘  │   │
│  │  字数统计: 0 / 10000                        [清空] [粘贴]      │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        情绪控制面板                              │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │   │
│  │  │ 预设情绪     │  │ 自定义调节   │  │ EMOD文件     │         │   │
│  │  │ ○ 中性      │  │ 愉悦度: ────●│  │ [选择文件]   │         │   │
│  │  │ ○ 快乐      │  │ 唤醒度: ●────│  │ 当前: 无     │         │   │
│  │  │ ○ 悲伤      │  │ 语速:   ──●──│  │              │         │   │
│  │  │ ○ 讲故事    │  │ 音高:   ●────│  │ [导出EMOD]   │         │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘         │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        音频播放器                                │   │
│  │  [▶] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 0:00 / 0:00  🔊───  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│                    [生成语音]              [下载音频]                   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 7. 后端架构设计

### 7.1 Rust网关服务架构

```rust
// src/main.rs

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::Arc;

mod api;
mod services;
mod python_bridge;
mod config;
mod error;

use crate::config::AppConfig;
use crate::services::{ModelService, TTSService, TrainingService, AnnotationService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    let config = AppConfig::from_env();
    
    // 初始化Python运行时
    pyo3::prepare_freethreaded_python();
    
    // 初始化服务
    let model_service = Arc::new(ModelService::new(&config.models_dir));
    let tts_service = Arc::new(TTSService::new(&config.tts_config));
    let training_service = Arc::new(TrainingService::new(&config.training_dir));
    let annotation_service = Arc::new(AnnotationService::new(&config.db_path));
    
    println!("🚀 AI朗读软件后端服务启动于 http://{}", config.server_addr);
    
    HttpServer::new(move || {
        App::new()
            // CORS配置
            .wrap(
                Cors::permissive()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
            )
            // 日志中间件
            .wrap(middleware::Logger::default())
            // 注册服务
            .app_data(web::Data::new(model_service.clone()))
            .app_data(web::Data::new(tts_service.clone()))
            .app_data(web::Data::new(training_service.clone()))
            .app_data(web::Data::new(annotation_service.clone()))
            // 注册路由
            .configure(api::routes::configure)
    })
    .bind(&config.server_addr)?
    .run()
    .await
}
```

### 7.2 Python桥接模块

```rust
// src/python_bridge/mod.rs

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::path::Path;

/// Python TTS引擎桥接
pub struct TTSBridge {
    module: Py<PyModule>,
}

impl TTSBridge {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let module = PyModule::import_bound(py, "tts_engine")?;
            Ok(Self { module })
        })
    }
    
    /// 调用TTS生成音频
    pub fn synthesize(
        &self,
        text: &str,
        model_path: &str,
        emod_path: Option<&str>,
        output_path: &str,
    ) -> PyResult<()> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            
            let kwargs = pyo3::types::PyDict::new_bound(py);
            kwargs.set_item("text", text)?;
            kwargs.set_item("model_path", model_path)?;
            kwargs.set_item("output_path", output_path)?;
            
            if let Some(emod) = emod_path {
                kwargs.set_item("emod_path", emod)?;
            }
            
            module.call_method("synthesize", (), Some(&kwargs))?;
            Ok(())
        })
    }
    
    /// 加载模型
    pub fn load_model(&self, model_path: &str) -> PyResult<String> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let result = module.call_method1("load_model", (model_path,))?;
            result.extract()
        })
    }
}

/// Python标注器桥接
pub struct AnnotatorBridge {
    module: Py<PyModule>,
}

impl AnnotatorBridge {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let module = PyModule::import_bound(py, "annotator")?;
            Ok(Self { module })
        })
    }
    
    /// 自动转录音频
    pub fn transcribe(&self, audio_path: &str) -> PyResult<TranscriptionResult> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let result = module.call_method1("transcribe", (audio_path,))?;
            
            let dict: &Bound<'_, PyDict> = result.downcast()?;
            
            Ok(TranscriptionResult {
                text: dict.get_item("text")?.unwrap().extract()?,
                segments: dict.get_item("segments")?.unwrap().extract()?,
            })
        })
    }
}

#[derive(Debug)]
pub struct TranscriptionResult {
    pub text: String,
    pub segments: Vec<Segment>,
}

#[derive(Debug)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}
```

### 7.3 API路由设计

```rust
// src/api/routes.rs

use actix_web::{web, Scope};
use crate::api::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // 模型管理
            .service(
                web::scope("/models")
                    .route("", web::get().to(handlers::models::list_models))
                    .route("", web::post().to(handlers::models::upload_model))
                    .route("/{id}", web::get().to(handlers::models::get_model))
                    .route("/{id}", web::delete().to(handlers::models::delete_model))
                    .route("/{id}/load", web::post().to(handlers::models::load_model))
            )
            // TTS合成
            .service(
                web::scope("/tts")
                    .route("/synthesize", web::post().to(handlers::tts::synthesize))
                    .route("/stream", web::post().to(handlers::tts::stream_synthesize))
            )
            // 训练管理
            .service(
                web::scope("/training")
                    .route("/start", web::post().to(handlers::training::start_training))
                    .route("/status/{id}", web::get().to(handlers::training::get_status))
                    .route("/cancel/{id}", web::post().to(handlers::training::cancel_training))
            )
            // 标注管理
            .service(
                web::scope("/annotations")
                    .route("", web::get().to(handlers::annotations::list_annotations))
                    .route("", web::post().to(handlers::annotations::create_annotation))
                    .route("/{id}", web::put().to(handlers::annotations::update_annotation))
                    .route("/transcribe", web::post().to(handlers::annotations::auto_transcribe))
                    .route("/export", web::post().to(handlers::annotations::export_annotations))
            )
            // 情绪管理
            .service(
                web::scope("/emotions")
                    .route("/presets", web::get().to(handlers::emotions::list_presets))
                    .route("", web::post().to(handlers::emotions::create_emod))
                    .route("/{id}", web::get().to(handlers::emotions::get_emod))
            )
    );
}
```

### 7.4 服务层实现

```rust
// src/services/tts_service.rs

use crate::python_bridge::TTSBridge;
use crate::error::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct TTSService {
    bridge: TTSBridge,
    loaded_models: Arc<RwLock<LruCache<String, ()>>>,
    output_dir: PathBuf,
}

impl TTSService {
    pub fn new(config: &TTSConfig) -> Self {
        let bridge = TTSBridge::new()
            .expect("Failed to initialize TTS bridge");
        
        let cache_size = NonZeroUsize::new(10).unwrap();
        
        Self {
            bridge,
            loaded_models: Arc::new(RwLock::new(LruCache::new(cache_size))),
            output_dir: config.output_dir.clone(),
        }
    }
    
    /// 合成语音
    pub async fn synthesize(
        &self,
        request: SynthesizeRequest,
    ) -> Result<SynthesizeResponse> {
        let output_path = self.output_dir
            .join(format!("output_{}.wav", uuid::Uuid::new_v4()));
        
        // 调用Python TTS引擎
        self.bridge.synthesize(
            &request.text,
            &request.model_path,
            request.emod_path.as_deref(),
            output_path.to_str().unwrap(),
        )?;
        
        Ok(SynthesizeResponse {
            audio_path: output_path.to_string_lossy().to_string(),
            duration: self.get_audio_duration(&output_path)?,
        })
    }
    
    /// 流式合成(用于长文本)
    pub async fn stream_synthesize(
        &self,
        request: SynthesizeRequest,
    ) -> Result<impl futures::Stream<Item = Result<bytes::Bytes>>> {
        // 分段处理长文本
        let segments = self.split_text(&request.text, 500);
        
        let stream = futures::stream::iter(segments)
            .then(move |segment| async move {
                // 逐段合成
                self.synthesize_segment(segment, &request).await
            });
        
        Ok(stream)
    }
    
    fn split_text(&self, text: &str, max_length: usize) -> Vec<String> {
        // 按句子分割，确保不超过最大长度
        let sentences: Vec<&str> = text.split_inclusive(&['。', '！', '？', '.', '!', '?'][..])
            .collect();
        
        let mut segments = Vec::new();
        let mut current = String::new();
        
        for sentence in sentences {
            if current.len() + sentence.len() > max_length && !current.is_empty() {
                segments.push(current.clone());
                current.clear();
            }
            current.push_str(sentence);
        }
        
        if !current.is_empty() {
            segments.push(current);
        }
        
        segments
    }
    
    fn get_audio_duration(&self, path: &PathBuf) -> Result<f64> {
        // 使用hound库读取WAV文件获取时长
        let reader = hound::WavReader::open(path)?;
        let duration = reader.duration() as f64 / reader.spec().sample_rate as f64;
        Ok(duration)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct SynthesizeRequest {
    pub text: String,
    pub model_path: String,
    pub emod_path: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct SynthesizeResponse {
    pub audio_path: String,
    pub duration: f64,
}
```

---

## 8. API接口规范

### 8.1 接口总览

| 模块 | 方法 | 端点 | 描述 |
|------|------|------|------|
| **模型管理** | GET | `/api/v1/models` | 获取模型列表 |
| | POST | `/api/v1/models` | 上传新模型 |
| | GET | `/api/v1/models/{id}` | 获取模型详情 |
| | DELETE | `/api/v1/models/{id}` | 删除模型 |
| | POST | `/api/v1/models/{id}/load` | 加载模型到内存 |
| **TTS合成** | POST | `/api/v1/tts/synthesize` | 文本转语音 |
| | POST | `/api/v1/tts/stream` | 流式合成(长文本) |
| **训练管理** | POST | `/api/v1/training/start` | 启动训练任务 |
| | GET | `/api/v1/training/status/{id}` | 查询训练状态 |
| | POST | `/api/v1/training/cancel/{id}` | 取消训练任务 |
| **标注管理** | GET | `/api/v1/annotations` | 获取标注列表 |
| | POST | `/api/v1/annotations` | 创建标注 |
| | PUT | `/api/v1/annotations/{id}` | 更新标注 |
| | POST | `/api/v1/annotations/transcribe` | 自动转录音频 |
| | POST | `/api/v1/annotations/export` | 导出标注数据 |
| **情绪管理** | GET | `/api/v1/emotions/presets` | 获取预设情绪 |
| | POST | `/api/v1/emotions` | 创建EMOD文件 |
| | GET | `/api/v1/emotions/{id}` | 获取EMOD详情 |

### 8.2 详细接口定义

#### 8.2.1 文本转语音接口

```yaml
POST /api/v1/tts/synthesize
Content-Type: application/json

Request Body:
{
    "text": "你好，这是一段测试文本。",
    "model_id": "voice_model_001",
    "emotion": {
        "preset": "storytelling",        # 使用预设情绪
        # 或者自定义参数:
        "custom": {
            "valence": 0.5,
            "arousal": 0.3,
            "speaking_rate": 0.9
        }
    },
    "output_format": "wav",              # wav/mp3/ogg
    "sample_rate": 22050
}

Response 200:
{
    "success": true,
    "data": {
        "audio_id": "audio_abc123",
        "audio_url": "/api/v1/audio/audio_abc123.wav",
        "duration": 3.5,
        "sample_rate": 22050,
        "format": "wav"
    }
}

Response 400:
{
    "success": false,
    "error": {
        "code": "INVALID_TEXT",
        "message": "文本长度超出限制"
    }
}
```

#### 8.2.2 启动训练接口

```yaml
POST /api/v1/training/start
Content-Type: application/json

Request Body:
{
    "model_name": "my_voice_model",
    "training_data": {
        "audio_files": [
            "/data/audio/sample1.wav",
            "/data/audio/sample2.wav"
        ],
        "annotations_file": "/data/annotations.json"
    },
    "config": {
        "epochs": 1000,
        "batch_size": 16,
        "learning_rate": 0.001,
        "save_every": 100,
        "validation_split": 0.1
    },
    "base_model": "pretrained_tacotron",   # 可选，基于预训练模型微调
    "emotion_extraction": true              # 是否提取情绪特征
}

Response 200:
{
    "success": true,
    "data": {
        "training_id": "train_xyz789",
        "status": "pending",
        "estimated_time": 3600
    }
}
```

#### 8.2.3 自动转录接口

```yaml
POST /api/v1/annotations/transcribe
Content-Type: multipart/form-data

Request:
- audio_file: (binary) 音频文件
- language: "zh" 语言代码
- word_timestamps: true 是否输出词级时间戳

Response 200:
{
    "success": true,
    "data": {
        "transcription_id": "trans_123",
        "text": "这是自动识别的文本内容。",
        "segments": [
            {
                "id": 1,
                "start": 0.0,
                "end": 1.5,
                "text": "这是自动",
                "confidence": 0.95
            },
            {
                "id": 2,
                "start": 1.5,
                "end": 3.2,
                "text": "识别的文本内容。",
                "confidence": 0.92
            }
        ],
        "language": "zh",
        "duration": 3.2
    }
}
```

---

## 9. 数据流与交互流程

### 9.1 模型训练流程

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        模型训练完整流程                                  │
│                                                                         │
│  用户操作                     系统处理                        输出      │
│  ─────────                    ─────────                      ─────     │
│                                                                         │
│  1. 上传训练音频  ────────→   接收并存储音频文件                         │
│     (wav/mp3)                                                           │
│                                                                         │
│  2. 自动标注      ────────→   Whisper ASR识别文本                       │
│     (可选手动修正)            ↓                                         │
│                              存入标注数据库                              │
│                                                                         │
│  3. 配置训练参数  ────────→   验证参数有效性                            │
│     (epochs, batch等)        ↓                                         │
│                              创建训练任务                                │
│                                                                         │
│  4. 启动训练      ────────→   数据预处理                                │
│                              ├── 音频特征提取                           │
│                              ├── 文本编码                               │
│                              └── 数据集划分                             │
│                              ↓                                         │
│                              模型训练                                   │
│                              ├── 前向传播                               │
│                              ├── 损失计算                               │
│                              ├── 反向传播                               │
│                              └── 参数更新                               │
│                              ↓                                         │
│                              情绪特征提取                               │
│                              └── 生成EMOD文件                           │
│                              ↓                                         │
│                              模型导出                                   │
│                              ├── model.pth                              │
│                              ├── features.index                         │
│                              └── emotion.emod                           │
│                                                                         │
│  5. 训练完成      ←────────   返回训练结果                              │
│                              ├── 模型文件路径                           │
│                              ├── 训练指标                               │
│                              └── 验证结果                               │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 9.2 文本朗读流程

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        文本朗读交互流程                                  │
│                                                                         │
│  前端                         后端                      TTS引擎         │
│  ────                         ────                      ──────         │
│                                                                         │
│  1. 用户输入文本                                                       │
│     选择模型/情绪                                                      │
│         │                                                              │
│         ▼                                                              │
│  2. 发送合成请求  ──────→  接收请求                                    │
│         │                      │                                       │
│         │                      ▼                                       │
│         │              验证参数                                        │
│         │              加载模型  ──────→  加载.pth/.index/.emod        │
│         │                      │                                       │
│         │                      ▼                                       │
│         │              调用TTS引擎  ──→  文本预处理                    │
│         │                      │         ├── 分词                      │
│         │                      │         ├── 音素转换                  │
│         │                      │         └── 韵律预测                  │
│         │                      │              │                        │
│         │                      │              ▼                        │
│         │                      │         声学模型推理                  │
│         │                      │         ├── 编码器                    │
│         │                      │         ├── 情绪注入                  │
│         │                      │         └── 解码器                    │
│         │                      │              │                        │
│         │                      │              ▼                        │
│         │                      │         声码器合成                    │
│         │                      │         └── 生成波形                  │
│         │                      │                                       │
│         │                      ▼                                       │
│         │              保存音频文件                                    │
│         │                      │                                       │
│         │                      ▼                                       │
│  3. 返回音频URL  ←──────  返回响应                                    │
│         │                                                              │
│         ▼                                                              │
│  4. 播放音频                                                          │
│     下载音频                                                           │
│                                                                        │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 10. 部署与运维

### 10.1 部署架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        部署架构图                                        │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        用户访问层                                │   │
│  │                         Nginx                                    │   │
│  │                    (反向代理 + 静态资源)                          │   │
│  └───────────────────────────────┬─────────────────────────────────┘   │
│                                  │                                      │
│         ┌────────────────────────┼────────────────────────┐            │
│         │                        │                        │            │
│         ▼                        ▼                        ▼            │
│  ┌─────────────┐          ┌─────────────┐          ┌─────────────┐    │
│  │  Rust API   │          │  Rust API   │          │  Rust API   │    │
│  │  Gateway    │          │  Gateway    │          │  Gateway    │    │
│  │  Instance 1 │          │  Instance 2 │          │  Instance N │    │
│  └──────┬──────┘          └──────┬──────┘          └──────┬──────┘    │
│         │                        │                        │            │
│         └────────────────────────┼────────────────────────┘            │
│                                  │                                      │
│                                  ▼                                      │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                      Python AI服务层                             │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐    │   │
│  │  │ TTS Engine│  │  Trainer  │  │ Annotator │  │  Feature  │    │   │
│  │  │  Service  │  │  Service  │  │  Service  │  │  Service  │    │   │
│  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘    │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                        存储层                                    │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐    │   │
│  │  │ 模型存储   │  │ 音频存储   │  │  数据库   │  │  缓存     │    │   │
│  │  │ (NFS/S3)  │  │ (NFS/S3)  │  │ (SQLite)  │  │ (Redis)   │    │   │
│  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘    │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

### 10.2 系统要求

| 组件 | 最低配置 | 推荐配置 |
|------|---------|---------|
| **CPU** | 4核心 | 8核心+ |
| **内存** | 8GB | 32GB+ |
| **GPU** | 无(可CPU推理) | NVIDIA RTX 3060 12GB+ |
| **存储** | 50GB SSD | 500GB NVMe SSD |
| **操作系统** | Linux/macOS/Windows | Ubuntu 22.04 LTS |

### 10.3 Docker部署配置

```dockerfile
# Dockerfile

# 阶段1: Python AI服务
FROM python:3.10-slim as python-base

WORKDIR /app

# 安装Python依赖
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libsndfile1 \
    && rm -rf /var/lib/apt/lists/*

# 复制Python代码
COPY python/ ./python/
WORKDIR /app/python

# 阶段2: Rust网关服务
FROM rust:1.75 as rust-builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# 编译Rust服务
RUN cargo build --release

# 阶段3: 最终镜像
FROM nvidia/cuda:11.8-runtime-ubuntu22.04

WORKDIR /app

# 复制Python环境
COPY --from=python-base /usr/local/lib/python3.10 /usr/local/lib/python3.10
COPY --from=python-base /app/python ./python

# 复制Rust二进制
COPY --from=rust-builder /app/target/release/ai-tts-server /usr/local/bin/

# 复制前端静态文件
COPY frontend/ ./frontend/

# 环境变量
ENV PYTHONPATH=/app/python
ENV MODELS_DIR=/data/models
ENV AUDIO_DIR=/data/audio

# 启动命令
CMD ["ai-tts-server"]
```

```yaml
# docker-compose.yml

version: '3.8'

services:
  api-gateway:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - ./data/models:/data/models
      - ./data/audio:/data/audio
    environment:
      - RUST_LOG=info
      - PYTHON_PATH=/app/python
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
    depends_on:
      - redis
  
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./frontend:/usr/share/nginx/html
      - ./nginx.conf:/etc/nginx/nginx.conf
    depends_on:
      - api-gateway

volumes:
  redis_data:
```

---

## 11. 技术选型总结

### 11.1 技术栈汇总

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        技术选型汇总表                                    │
├─────────────────┬───────────────────────────────────────────────────────┤
│ 层级            │ 技术选型                                              │
├─────────────────┼───────────────────────────────────────────────────────┤
│ 前端框架        │ 纯HTML5 + CSS3 + ES6+ Modules (无第三方框架)          │
│ 前端组件        │ Web Components (原生自定义元素)                       │
│ 音频处理        │ Web Audio API (原生浏览器API)                         │
├─────────────────┼───────────────────────────────────────────────────────┤
│ 后端框架        │ Rust + Actix-web / Axum                               │
│ Python桥接      │ PyO3 (Rust-Python互操作)                              │
│ 异步运行时      │ Tokio (Rust异步)                                      │
├─────────────────┼───────────────────────────────────────────────────────┤
│ TTS引擎         │ 自研Emo-Tacotron + HiFi-GAN (混合方案)                │
│ 深度学习框架    │ PyTorch 2.x                                           │
│ 音频处理        │ librosa, soundfile, torchaudio                        │
│ ASR识别         │ OpenAI Whisper                                        │
├─────────────────┼───────────────────────────────────────────────────────┤
│ 数据库          │ SQLite (标注数据)                                     │
│ 缓存            │ Redis / DashMap (内存缓存)                            │
│ 文件存储        │ 本地文件系统 / NFS / S3                               │
├─────────────────┼───────────────────────────────────────────────────────┤
│ 模型格式        │ .pth (PyTorch权重) + .index (特征索引) + .emod (情绪) │
│ 音频格式        │ WAV (主要) / MP3 / FLAC                               │
├─────────────────┼───────────────────────────────────────────────────────┤
│ 容器化          │ Docker + Docker Compose                               │
│ 反向代理        │ Nginx                                                 │
└─────────────────┴───────────────────────────────────────────────────────┘
```

### 11.2 核心创新点

1. **EMOD情绪控制文件**: 独创的情绪描述文件格式，实现精细化情感控制
2. **混合TTS架构**: 神经网络前端 + 参数化声码器，兼顾音质与可控性
3. **Rust+Python混合架构**: 高性能网关 + 成熟AI生态的最佳结合
4. **原生前端技术栈**: 无框架依赖，轻量高效，易于维护
5. **自动标注+人工修正**: Whisper ASR自动识别，支持可视化编辑修正

---

## 附录

### A. 项目目录结构

```
ai-tts-software/
├── frontend/                    # 前端代码
│   ├── index.html
│   ├── styles/
│   ├── scripts/
│   └── assets/
│
├── src/                         # Rust后端代码
│   ├── main.rs
│   ├── api/
│   ├── services/
│   ├── python_bridge/
│   └── config/
│
├── python/                      # Python AI模块
│   ├── tts_engine/
│   ├── trainer/
│   ├── annotator/
│   └── utils/
│
├── models/                      # 模型存储目录
├── data/                        # 数据存储目录
├── config/                      # 配置文件目录
│
├── Dockerfile
├── docker-compose.yml
├── Cargo.toml                   # Rust依赖
├── requirements.txt             # Python依赖
└── README.md
```

### B. 参考资料

1. Tacotron 2: Natural TTS Synthesis by Conditioning WaveNet on Mel Spectrogram Predictions
2. HiFi-GAN: Generative Adversarial Networks for Efficient and High Fidelity Speech Synthesis
3. EmotiVoice: A Multi-Voice Emotional TTS Engine
4. OpenAI Whisper: Robust Speech Recognition via Large-Scale Weak Supervision
5. PyO3: Rust bindings for Python

---

**文档版本**: v1.0  
**最后更新**: 2025-05-10
