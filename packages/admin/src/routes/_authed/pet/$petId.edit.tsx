import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useEffect, useState } from 'react';
import {
  createPet,
  getPet,
  listPetBreeds,
  listPersonalityTags,
  updatePet,
  updatePetAppearance,
  updatePetPersonality,
  type BreedDto,
  type PersonalityTagCategory,
  type PersonalityTagSimple,
} from '@/services';

export const Route = createFileRoute('/_authed/pet/$petId/edit')({
  component: PetEditPage,
});

interface FormState {
  name: string;
  emoji: string;
  gender: string;
  birthYear: string;
  birthMonth: string;
  birthApproximate: boolean;
  breedId: string;
  coatColor: string;
  coatPattern: string;
  neuterStatus: string;
  personalityTagIds: string[];
  customTags: string[];
}

const COAT_COLORS: Array<{ value: string; label: string }> = [
  { value: 'cream', label: '奶油' },
  { value: 'tan', label: '棕色' },
  { value: 'brown', label: '深棕' },
  { value: 'black', label: '黑色' },
  { value: 'white', label: '白色' },
  { value: 'gray', label: '灰色' },
  { value: 'gold', label: '金色' },
  { value: 'red', label: '红色' },
  { value: 'choco', label: '巧克力' },
  { value: 'merle', label: '陨石' },
  { value: 'fawn', label: '浅黄' },
  { value: 'pearl', label: '珍珠' },
];

const COAT_PATTERNS: Array<{ value: string; label: string }> = [
  { value: '纯色', label: '纯色' },
  { value: '虎斑', label: '虎斑' },
  { value: '陨石', label: '陨石' },
  { value: '双色', label: '双色' },
  { value: '三色', label: '三色' },
  { value: '花斑', label: '花斑' },
];

function PetEditPage() {
  const { petId } = Route.useParams();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const isNew = petId === 'new';

  const [form, setForm] = useState<FormState>({
    name: '',
    emoji: '🐾',
    gender: 'male',
    birthYear: String(new Date().getFullYear()),
    birthMonth: '',
    birthApproximate: false,
    breedId: '',
    coatColor: 'gold',
    coatPattern: '纯色',
    neuterStatus: 'intact',
    personalityTagIds: [],
    customTags: [],
  });
  const [customTagInput, setCustomTagInput] = useState('');
  const [breedKeyword, setBreedKeyword] = useState('');
  const [formError, setFormError] = useState<string | null>(null);

  // 编辑模式拉取宠物详情，再初始化表单。
  const petQuery = useQuery({
    queryKey: ['pet', petId],
    queryFn: async () => {
      const result = await getPet({ path: { petId } });
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '加载宠物详情失败');
      }
      return result.data.data;
    },
    enabled: !isNew,
    select: (data) => data,
  });

  // 当详情数据首次到达时把字段同步进 form。useEffect 而非 useMemo，
  // 避免 render 过程中触发 setState 警告。
  useEffect(() => {
    if (!petQuery.data) return;
    const detail = petQuery.data;
    setForm({
      name: detail.name,
      emoji: detail.emoji ?? '🐾',
      gender: detail.gender,
      birthYear: String(detail.birthYear),
      birthMonth: detail.birthMonth ? String(detail.birthMonth) : '',
      birthApproximate: detail.birthApproximate,
      breedId: detail.breedId,
      coatColor: detail.coatColor,
      coatPattern: detail.coatPattern ?? '纯色',
      neuterStatus: detail.neuterStatus,
      personalityTagIds: detail.personalityTags.map((tag) => tag.tagId),
      customTags: [...detail.customTags],
    });
  }, [petQuery.data]);

  const breedsQuery = useQuery({
    queryKey: ['pet-breeds', breedKeyword],
    queryFn: async () => {
      const result = await listPetBreeds({ query: { keyword: breedKeyword || undefined, page: 1, pageSize: 50 } });
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '加载品种失败');
      }
      return result.data.data;
    },
  });

  const personalityQuery = useQuery({
    queryKey: ['personality-tags'],
    queryFn: async () => {
      const result = await listPersonalityTags();
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '加载性格标签失败');
      }
      return result.data.data;
    },
  });

  const createMutation = useMutation({
    mutationFn: async () => {
      const result = await createPet({
        body: {
          name: form.name.trim(),
          emoji: form.emoji || undefined,
          gender: form.gender,
          birthYear: Number(form.birthYear),
          birthMonth: form.birthMonth ? Number(form.birthMonth) : undefined,
          birthApproximate: form.birthApproximate,
          breedId: form.breedId,
          coatColor: form.coatColor,
          coatPattern: form.coatPattern || undefined,
          neuterStatus: form.neuterStatus,
          personalityTagIds: form.personalityTagIds,
          customTags: form.customTags,
        },
      });
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '创建失败');
      }
      return result.data.data;
    },
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['pets'] });
      void navigate({ to: '/pet' });
    },
    onError: (error: Error) => setFormError(error.message),
  });

  // 编辑模式：三个分段更新（基础信息、外貌、性格），失败时把第一个错误透出。
  const updateMutation = useMutation({
    mutationFn: async () => {
      const base = {
        path: { petId },
      };
      const basicResult = await updatePet({
        ...base,
        body: {
          name: form.name.trim(),
          gender: form.gender,
          birthYear: Number(form.birthYear),
          birthMonth: form.birthMonth ? Number(form.birthMonth) : null,
          birthApproximate: form.birthApproximate,
          neuterStatus: form.neuterStatus,
        },
      });
      if (basicResult.error || !basicResult.data) {
        throw new Error(basicResult.error?.message ?? '更新基础信息失败');
      }
      const appearanceResult = await updatePetAppearance({
        ...base,
        body: {
          breedId: form.breedId,
          coatColor: form.coatColor,
          coatPattern: form.coatPattern,
        },
      });
      if (appearanceResult.error || !appearanceResult.data) {
        throw new Error(appearanceResult.error?.message ?? '更新外貌失败');
      }
      const personalityResult = await updatePetPersonality({
        ...base,
        body: {
          personalityTagIds: form.personalityTagIds,
          customTags: form.customTags,
        },
      });
      if (personalityResult.error || !personalityResult.data) {
        throw new Error(personalityResult.error?.message ?? '更新性格标签失败');
      }
    },
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['pets'] });
      void queryClient.invalidateQueries({ queryKey: ['pet', petId] });
      void navigate({ to: '/pet' });
    },
    onError: (error: Error) => setFormError(error.message),
  });

  function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (createMutation.isPending || updateMutation.isPending) return;
    setFormError(null);

    if (form.name.trim().length === 0) {
      setFormError('名字必填');
      return;
    }
    if (!form.breedId) {
      setFormError('请选择品种');
      return;
    }
    if (isNew) {
      void createMutation.mutate();
    } else {
      void updateMutation.mutate();
    }
  }

  function togglePersonalityTag(tagId: string) {
    setForm((current) => {
      const exists = current.personalityTagIds.includes(tagId);
      return {
        ...current,
        personalityTagIds: exists
          ? current.personalityTagIds.filter((id) => id !== tagId)
          : [...current.personalityTagIds, tagId],
      };
    });
  }

  function addCustomTag() {
    const trimmed = customTagInput.trim();
    if (!trimmed) return;
    if (form.customTags.includes(trimmed)) {
      setCustomTagInput('');
      return;
    }
    setForm((current) => ({ ...current, customTags: [...current.customTags, trimmed] }));
    setCustomTagInput('');
  }

  function removeCustomTag(tag: string) {
    setForm((current) => ({
      ...current,
      customTags: current.customTags.filter((item) => item !== tag),
    }));
  }

  const breeds: BreedDto[] = breedsQuery.data?.list ?? [];
  const categories: PersonalityTagCategory[] = personalityQuery.data?.categories ?? [];
  const submitting = createMutation.isPending || updateMutation.isPending;
  const selectedBreed = breeds.find((breed) => breed.breedId === form.breedId);

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-slate-900">
          {isNew ? '新建宠物' : '编辑宠物'}
        </h1>
        <button
          type="button"
          onClick={() => navigate({ to: '/pet' })}
          className="text-sm text-slate-600 transition hover:text-slate-900"
        >
          返回列表
        </button>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        {/* 基础信息 */}
        <section className="rounded-xl bg-white p-6 ring-1 ring-slate-200">
          <h2 className="text-base font-medium text-slate-900">基础信息</h2>
          <div className="mt-4 grid grid-cols-1 gap-4 md:grid-cols-2">
            <Field label="名字（1-10 字符）" required>
              <input
                type="text"
                value={form.name}
                onChange={(event) => setForm((current) => ({ ...current, name: event.target.value }))}
                maxLength={10}
                disabled={submitting}
                className="input-base"
                placeholder="如：豆豆"
              />
            </Field>
            <Field label="emoji">
              <input
                type="text"
                value={form.emoji}
                onChange={(event) => setForm((current) => ({ ...current, emoji: event.target.value }))}
                disabled={submitting}
                className="input-base"
                placeholder="🐾"
              />
            </Field>
            <Field label="性别" required>
              <select
                value={form.gender}
                onChange={(event) => setForm((current) => ({ ...current, gender: event.target.value }))}
                disabled={submitting}
                className="input-base"
              >
                <option value="male">公</option>
                <option value="female">母</option>
              </select>
            </Field>
            <Field label="绝育状态" required>
              <select
                value={form.neuterStatus}
                onChange={(event) => setForm((current) => ({ ...current, neuterStatus: event.target.value }))}
                disabled={submitting}
                className="input-base"
              >
                <option value="intact">未绝育</option>
                <option value="neutered">已绝育</option>
                <option value="planned">计划中</option>
              </select>
            </Field>
            <Field label="出生年份" required>
              <input
                type="number"
                value={form.birthYear}
                onChange={(event) => setForm((current) => ({ ...current, birthYear: event.target.value }))}
                disabled={submitting}
                className="input-base"
                placeholder="2023"
              />
            </Field>
            <Field label="出生月份">
              <input
                type="number"
                min={1}
                max={12}
                value={form.birthMonth}
                onChange={(event) => setForm((current) => ({ ...current, birthMonth: event.target.value }))}
                disabled={submitting}
                className="input-base"
                placeholder="未知可不填"
              />
            </Field>
            <Field label="出生日期是否大概">
              <label className="flex items-center gap-2 text-sm text-slate-700">
                <input
                  type="checkbox"
                  checked={form.birthApproximate}
                  onChange={(event) => setForm((current) => ({ ...current, birthApproximate: event.target.checked }))}
                  disabled={submitting}
                  className="h-4 w-4 rounded border-slate-300"
                />
                领养或生日未知时勾选
              </label>
            </Field>
          </div>
        </section>

        {/* 品种与外貌 */}
        <section className="rounded-xl bg-white p-6 ring-1 ring-slate-200">
          <h2 className="text-base font-medium text-slate-900">品种与外貌</h2>
          <div className="mt-4 space-y-4">
            <Field label="品种" required>
              <input
                type="text"
                value={breedKeyword}
                onChange={(event) => setBreedKeyword(event.target.value)}
                disabled={submitting}
                className="input-base"
                placeholder="按名称或拼音首字母搜索，如 金毛 / J"
              />
              {breedsQuery.isLoading ? (
                <p className="mt-1 text-xs text-slate-400">搜索中…</p>
              ) : (
                <div className="mt-2 max-h-48 overflow-y-auto rounded-md border border-slate-200">
                  {breeds.length === 0 ? (
                    <p className="px-3 py-2 text-xs text-slate-400">输入关键词后展示候选品种</p>
                  ) : (
                    breeds.map((breed) => (
                      <button
                        key={breed.breedId}
                        type="button"
                        onClick={() => {
                          setForm((current) => ({ ...current, breedId: breed.breedId }));
                          setBreedKeyword(breed.name);
                        }}
                        className={[
                          'flex w-full items-center justify-between px-3 py-2 text-left text-sm transition hover:bg-slate-50',
                          form.breedId === breed.breedId ? 'bg-slate-100 text-slate-900' : 'text-slate-700',
                        ].join(' ')}
                      >
                        <span>
                          {breed.name}
                          {breed.sizeCategory && (
                            <span className="ml-2 text-xs text-slate-400">{breed.sizeCategory}</span>
                          )}
                        </span>
                      </button>
                    ))
                  )}
                </div>
              )}
              {selectedBreed && (
                <p className="mt-1 text-xs text-slate-500">
                  已选：{selectedBreed.name}
                </p>
              )}
            </Field>

            <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
              <Field label="毛色" required>
                <select
                  value={form.coatColor}
                  onChange={(event) => setForm((current) => ({ ...current, coatColor: event.target.value }))}
                  disabled={submitting}
                  className="input-base"
                >
                  {COAT_COLORS.map((color) => (
                    <option key={color.value} value={color.value}>
                      {color.label}（{color.value}）
                    </option>
                  ))}
                </select>
              </Field>
              <Field label="花纹">
                <select
                  value={form.coatPattern}
                  onChange={(event) => setForm((current) => ({ ...current, coatPattern: event.target.value }))}
                  disabled={submitting}
                  className="input-base"
                >
                  {COAT_PATTERNS.map((pattern) => (
                    <option key={pattern.value} value={pattern.value}>
                      {pattern.label}
                    </option>
                  ))}
                </select>
              </Field>
            </div>
          </div>
        </section>

        {/* 性格标签 */}
        <section className="rounded-xl bg-white p-6 ring-1 ring-slate-200">
          <h2 className="text-base font-medium text-slate-900">性格标签</h2>
          {personalityQuery.isLoading ? (
            <p className="mt-3 text-sm text-slate-400">加载中…</p>
          ) : personalityQuery.isError ? (
            <p className="mt-3 text-sm text-red-600">
              {(personalityQuery.error as Error)?.message ?? '加载失败'}
            </p>
          ) : (
            <div className="mt-4 space-y-4">
              {categories.map((category) => (
                <div key={category.categoryId}>
                  <p className="text-sm font-medium text-slate-700">{category.categoryName}</p>
                  <div className="mt-2 flex flex-wrap gap-2">
                    {category.tags.map((tag) => (
                      <TagChip
                        key={tag.tagId}
                        tag={tag}
                        selected={form.personalityTagIds.includes(tag.tagId)}
                        onClick={() => togglePersonalityTag(tag.tagId)}
                        disabled={submitting}
                      />
                    ))}
                  </div>
                </div>
              ))}

              <div>
                <p className="text-sm font-medium text-slate-700">自定义标签</p>
                <div className="mt-2 flex flex-wrap gap-2">
                  {form.customTags.map((tag) => (
                    <span
                      key={tag}
                      className="inline-flex items-center gap-1 rounded-full bg-slate-900 px-3 py-1 text-xs text-white"
                    >
                      {tag}
                      <button
                        type="button"
                        onClick={() => removeCustomTag(tag)}
                        className="text-white/70 hover:text-white"
                        disabled={submitting}
                      >
                        ×
                      </button>
                    </span>
                  ))}
                </div>
                <div className="mt-2 flex gap-2">
                  <input
                    type="text"
                    value={customTagInput}
                    onChange={(event) => setCustomTagInput(event.target.value)}
                    onKeyDown={(event) => {
                      if (event.key === 'Enter') {
                        event.preventDefault();
                        addCustomTag();
                      }
                    }}
                    disabled={submitting}
                    className="input-base flex-1"
                    placeholder="输入后回车添加，如 爱追球"
                  />
                  <button
                    type="button"
                    onClick={addCustomTag}
                    className="rounded-md bg-slate-100 px-3 py-2 text-sm text-slate-700 transition hover:bg-slate-200"
                    disabled={submitting || customTagInput.trim().length === 0}
                  >
                    添加
                  </button>
                </div>
              </div>
            </div>
          )}
        </section>

        {formError && (
          <p className="text-sm text-red-600">{formError}</p>
        )}

        <div className="flex justify-end gap-2">
          <button
            type="button"
            onClick={() => navigate({ to: '/pet' })}
            className="rounded-md px-3 py-2 text-sm text-slate-600 transition hover:bg-slate-100"
            disabled={submitting}
          >
            取消
          </button>
          <button
            type="submit"
            className="rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white transition hover:bg-slate-700 disabled:opacity-60"
            disabled={submitting}
          >
            {submitting ? '提交中…' : isNew ? '创建宠物' : '保存修改'}
          </button>
        </div>
      </form>
    </div>
  );
}

interface FieldProps {
  label: string;
  required?: boolean;
  children: React.ReactNode;
}

function Field({ label, required, children }: FieldProps) {
  return (
    <label className="block">
      <span className="block text-sm font-medium text-slate-700">
        {label}
        {required && <span className="ml-1 text-red-500">*</span>}
      </span>
      <div className="mt-1">{children}</div>
    </label>
  );
}

interface TagChipProps {
  tag: PersonalityTagSimple;
  selected: boolean;
  onClick: () => void;
  disabled?: boolean;
}

function TagChip({ tag, selected, onClick, disabled }: TagChipProps) {
  return (
    <button
      key={tag.tagId}
      type="button"
      onClick={onClick}
      disabled={disabled}
      className={[
        'rounded-full px-3 py-1 text-xs transition',
        selected
          ? 'bg-slate-900 text-white'
          : 'bg-slate-100 text-slate-700 hover:bg-slate-200',
        disabled ? 'cursor-not-allowed opacity-60' : '',
      ].join(' ')}
    >
      {tag.name}
    </button>
  );
}
