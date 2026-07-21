import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import {
  createBreed,
  deleteBreed,
  listPetBreeds,
  type BreedDto,
} from '@/services';
import { getAuthToken } from '@/lib/auth';

export const Route = createFileRoute('/_authed/breed/')({
  component: BreedManagePage,
});

const SPECIES_OPTIONS: { value: string; label: string }[] = [
  { value: 'dog', label: '犬' },
  { value: 'cat', label: '猫' },
  { value: 'rabbit', label: '兔' },
  { value: 'bird', label: '鸟' },
  { value: 'rodent', label: '啮齿' },
  { value: 'reptile', label: '爬行' },
  { value: 'fish', label: '鱼' },
  { value: 'other', label: '其他' },
];

interface CreateForm {
  name: string;
  nameCn: string;
  sizeCategory: string;
  coatType: string;
  origin: string;
}

const EMPTY_FORM: CreateForm = {
  name: '',
  nameCn: '',
  sizeCategory: '',
  coatType: '',
  origin: '',
};

function BreedManagePage() {
  const queryClient = useQueryClient();
  const [species, setSpecies] = useState('dog');
  const [keyword, setKeyword] = useState('');
  const [showCreate, setShowCreate] = useState(false);
  const [form, setForm] = useState<CreateForm>(EMPTY_FORM);
  const [formError, setFormError] = useState<string | null>(null);
  const [pendingDeleteId, setPendingDeleteId] = useState<string | null>(null);

  const breedsQuery = useQuery({
    queryKey: ['breeds', species, keyword],
    queryFn: async () => {
      const result = await listPetBreeds({
        query: {
          species,
          keyword: keyword || undefined,
          page: 1,
          pageSize: 100,
        },
      });
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '加载品种列表失败');
      }
      return result.data.data;
    },
  });

  const createMutation = useMutation({
    mutationFn: async () => {
      const result = await createBreed({
        body: {
          species,
          name: form.name,
          nameCn: form.nameCn,
          sizeCategory: form.sizeCategory || undefined,
          coatType: form.coatType || undefined,
          origin: form.origin || undefined,
        },
      });
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '创建失败');
      }
      return result.data.data;
    },
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['breeds'] });
      setForm(EMPTY_FORM);
      setFormError(null);
      setShowCreate(false);
    },
    onError: (error: Error) => {
      setFormError(error.message);
    },
  });

  const deleteMutation = useMutation({
    mutationFn: async (breedId: string) => {
      const result = await deleteBreed({ path: { breedId } });
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '删除失败');
      }
    },
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['breeds'] });
      setPendingDeleteId(null);
    },
    onError: (error: Error) => {
      window.alert(error.message);
      setPendingDeleteId(null);
    },
  });

  async function handleExport() {
    // exportBreeds 返回原始 JSON 文件，需手动处理下载
    const token = getAuthToken();
    const res = await fetch('/api/admin/breeds/export', {
      headers: token ? { Authorization: `Bearer ${token}` } : {},
    });
    if (!res.ok) {
      window.alert('导出失败');
      return;
    }
    const blob = await res.blob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'breeds.json';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  const breeds: BreedDto[] = breedsQuery.data?.list ?? [];

  function submitCreate(event: React.FormEvent) {
    event.preventDefault();
    if (!form.name.trim() || !form.nameCn.trim()) {
      setFormError('英文名和中文名必填');
      return;
    }
    void createMutation.mutate();
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-slate-900">品种管理</h1>
        <div className="flex gap-2">
          <button
            type="button"
            onClick={handleExport}
            className="rounded-lg border border-slate-300 px-3 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50"
          >
            导出 seed JSON
          </button>
          <button
            type="button"
            onClick={() => {
              setForm(EMPTY_FORM);
              setFormError(null);
              setShowCreate(true);
            }}
            className="rounded-lg bg-slate-900 px-3 py-2 text-sm font-medium text-white transition hover:bg-slate-700"
          >
            新增品种
          </button>
        </div>
      </div>

      <div className="flex flex-wrap items-center gap-3">
        <select
          value={species}
          onChange={(e) => setSpecies(e.target.value)}
          className="rounded-lg border border-slate-300 px-3 py-2 text-sm"
        >
          {SPECIES_OPTIONS.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
        <input
          type="text"
          value={keyword}
          onChange={(e) => setKeyword(e.target.value)}
          placeholder="按名称搜索"
          className="rounded-lg border border-slate-300 px-3 py-2 text-sm"
        />
        <span className="text-xs text-slate-500">
          共 {breedsQuery.data?.total ?? 0} 条
        </span>
      </div>

      <div className="rounded-xl bg-white ring-1 ring-slate-200">
        {breedsQuery.isLoading ? (
          <p className="p-6 text-sm text-slate-500">加载中…</p>
        ) : breedsQuery.isError ? (
          <p className="p-6 text-sm text-red-600">
            {(breedsQuery.error as Error)?.message ?? '加载失败'}
          </p>
        ) : breeds.length === 0 ? (
          <p className="p-6 text-sm text-slate-500">该物种暂无品种数据。</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 text-left text-xs font-medium uppercase text-slate-500">
                  <th className="px-4 py-3">ID</th>
                  <th className="px-4 py-3">英文名</th>
                  <th className="px-4 py-3">中文名</th>
                  <th className="px-4 py-3">体型</th>
                  <th className="px-4 py-3">毛型</th>
                  <th className="px-4 py-3">起源</th>
                  <th className="px-4 py-3 text-right">操作</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100">
                {breeds.map((breed) => (
                  <tr key={breed.breedId} className="hover:bg-slate-50">
                    <td className="px-4 py-3 font-mono text-xs text-slate-500">
                      {breed.breedId}
                    </td>
                    <td className="px-4 py-3 font-medium text-slate-900">
                      {breed.name}
                    </td>
                    <td className="px-4 py-3 text-slate-600">{breed.nameCn}</td>
                    <td className="px-4 py-3 text-slate-600">
                      {breed.sizeCategory ?? '-'}
                    </td>
                    <td className="px-4 py-3 text-slate-600">
                      {breed.coatType ?? '-'}
                    </td>
                    <td className="px-4 py-3 text-slate-600">
                      {breed.origin ?? '-'}
                    </td>
                    <td className="px-4 py-3 text-right">
                      <button
                        type="button"
                        onClick={() => setPendingDeleteId(breed.breedId)}
                        className="text-red-600 transition hover:text-red-700"
                      >
                        删除
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {showCreate && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4">
          <div className="w-full max-w-md rounded-xl bg-white p-6 shadow-lg ring-1 ring-slate-200">
            <h2 className="text-lg font-semibold text-slate-900">
              新增品种 · {SPECIES_OPTIONS.find((o) => o.value === species)?.label}
            </h2>
            <form onSubmit={submitCreate} className="mt-4 space-y-3">
              <div>
                <label className="block text-sm font-medium text-slate-700">英文名 *</label>
                <input
                  type="text"
                  value={form.name}
                  onChange={(e) => setForm({ ...form, name: e.target.value })}
                  className="mt-1 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm"
                  placeholder="Afghan Hound"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-slate-700">中文名 *</label>
                <input
                  type="text"
                  value={form.nameCn}
                  onChange={(e) => setForm({ ...form, nameCn: e.target.value })}
                  className="mt-1 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm"
                  placeholder="阿富汗猎犬"
                />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <label className="block text-sm font-medium text-slate-700">体型</label>
                  <input
                    type="text"
                    value={form.sizeCategory}
                    onChange={(e) => setForm({ ...form, sizeCategory: e.target.value })}
                    className="mt-1 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm"
                    placeholder="small/medium/large"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-slate-700">毛型</label>
                  <input
                    type="text"
                    value={form.coatType}
                    onChange={(e) => setForm({ ...form, coatType: e.target.value })}
                    className="mt-1 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm"
                    placeholder="short/long/wire"
                  />
                </div>
              </div>
              <div>
                <label className="block text-sm font-medium text-slate-700">起源地</label>
                <input
                  type="text"
                  value={form.origin}
                  onChange={(e) => setForm({ ...form, origin: e.target.value })}
                  className="mt-1 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm"
                  placeholder="Germany"
                />
              </div>
              {formError && <p className="text-sm text-red-600">{formError}</p>}
              <div className="flex justify-end gap-2 pt-2">
                <button
                  type="button"
                  onClick={() => setShowCreate(false)}
                  className="rounded-md px-3 py-1.5 text-sm text-slate-600 transition hover:bg-slate-100"
                  disabled={createMutation.isPending}
                >
                  取消
                </button>
                <button
                  type="submit"
                  className="rounded-md bg-slate-900 px-3 py-1.5 text-sm font-medium text-white transition hover:bg-slate-700 disabled:opacity-60"
                  disabled={createMutation.isPending}
                >
                  {createMutation.isPending ? '创建中…' : '创建'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {pendingDeleteId && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4">
          <div className="w-full max-w-sm rounded-xl bg-white p-6 shadow-lg ring-1 ring-slate-200">
            <h2 className="text-lg font-semibold text-slate-900">删除品种</h2>
            <p className="mt-2 text-sm text-slate-600">
              删除后该品种将无法在宠物档案中被选择，确认删除吗？
            </p>
            <div className="mt-4 flex justify-end gap-2">
              <button
                type="button"
                onClick={() => setPendingDeleteId(null)}
                className="rounded-md px-3 py-1.5 text-sm text-slate-600 transition hover:bg-slate-100"
                disabled={deleteMutation.isPending}
              >
                取消
              </button>
              <button
                type="button"
                onClick={() => void deleteMutation.mutate(pendingDeleteId)}
                className="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition hover:bg-red-700 disabled:opacity-60"
                disabled={deleteMutation.isPending}
              >
                {deleteMutation.isPending ? '删除中…' : '确认删除'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
