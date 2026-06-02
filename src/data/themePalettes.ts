export type AccentTheme = 'green' | 'blue' | 'violet' | 'rose' | 'amber';

export interface ThemePalette {
  id: AccentTheme;
  name: string;
  accent: string;
  accent2: string;
  soft: string;
  border: string;
  text: string;
}

export const themePalettes: ThemePalette[] = [
  {
    id: 'green',
    name: '直播绿',
    accent: '#84F042',
    accent2: '#2DD4BF',
    soft: 'rgba(132, 240, 66, 0.14)',
    border: 'rgba(132, 240, 66, 0.38)',
    text: '#D9FFB3',
  },
  {
    id: 'blue',
    name: '电光蓝',
    accent: '#3B82F6',
    accent2: '#22D3EE',
    soft: 'rgba(59, 130, 246, 0.16)',
    border: 'rgba(59, 130, 246, 0.42)',
    text: '#BFDBFE',
  },
  {
    id: 'violet',
    name: '紫罗兰',
    accent: '#A78BFA',
    accent2: '#F472B6',
    soft: 'rgba(167, 139, 250, 0.15)',
    border: 'rgba(167, 139, 250, 0.4)',
    text: '#DDD6FE',
  },
  {
    id: 'rose',
    name: '玫瑰粉',
    accent: '#F472B6',
    accent2: '#FB7185',
    soft: 'rgba(244, 114, 182, 0.15)',
    border: 'rgba(244, 114, 182, 0.42)',
    text: '#FBCFE8',
  },
  {
    id: 'amber',
    name: '暖琥珀',
    accent: '#FBBF24',
    accent2: '#FB923C',
    soft: 'rgba(251, 191, 36, 0.15)',
    border: 'rgba(251, 191, 36, 0.42)',
    text: '#FDE68A',
  },
];

export const getThemePalette = (id?: AccentTheme) => {
  return themePalettes.find((palette) => palette.id === id) || themePalettes[0];
};
