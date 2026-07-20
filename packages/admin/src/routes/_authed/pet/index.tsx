import { createFileRoute, Link } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import {
  deletePet,
  listPets,
  type PetListItem,
} from '@/services';

export const Route = createFileRoute('/_authed/pet/')({
  component: PetListPage,
});

function PetListPage() {
  const queryClient = useQueryClient();
  const [pendingDeleteId, setPendingDeleteId] = useState<string | null>(null);

  const petsQuery = useQuery({
    queryKey: ['pets'],
    queryFn: async () => {
      const result = await listPets();
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '加载宠物列表失败');
      }
      return result.data.data;
    },
  });

  const deleteMutation = useMutation({
    mutationFn: async (petId: string) => {
      const result = await deletePet({ path: { petId } });
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? '删除失败');
      }
      return result.data.data;
    },
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ['pets'] });
      setPendingDeleteId(null);
    },
    onError: (error: Error) => {
      window.alert(error.message);
      setPendingDeleteId(null);
    },
  });

  const pets: PetListItem[] = petsQuery.data?.list ?? [];

  function handleDelete(petId: string) {
    setPendingDeleteId(petId);
  }

  function confirmDelete() {
    if (pendingDeleteId) {
      void deleteMutation.mutate(pendingDeleteId);
    }
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-slate-900">宠物档案</h1>
        <Link
          to="/pet/$petId/edit"
          params={{ petId: 'new' }}
          className="rounded-lg bg-slate-900 px-3 py-2 text-sm font-medium text-white transition hover:bg-slate-700"
        >
          新建宠物
        </Link>
      </div>

      <div className="rounded-xl bg-white ring-1 ring-slate-200">
        {petsQuery.isLoading ? (
          <p className="p-6 text-sm text-slate-500">加载中…</p>
        ) : petsQuery.isError ? (
          <p className="p-6 text-sm text-red-600">
            {(petsQuery.error as Error)?.message ?? '加载失败'}
          </p>
        ) : pets.length === 0 ? (
          <p className="p-6 text-sm text-slate-500">
            还没有宠物。点击右上角"新建宠物"开始建档。
          </p>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 text-left text-xs font-medium uppercase text-slate-500">
                  <th className="px-4 py-3">名字</th>
                  <th className="px-4 py-3">品种</th>
                  <th className="px-4 py-3">性别</th>
                  <th className="px-4 py-3">生日</th>
                  <th className="px-4 py-3">绝育</th>
                  <th className="px-4 py-3">陪伴天数</th>
                  <th className="px-4 py-3 text-right">操作</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100">
                {pets.map((pet) => (
                  <tr key={pet.petId} className="hover:bg-slate-50">
                    <td className="px-4 py-3 font-medium text-slate-900">
                      <span className="mr-1">{pet.avatar ?? '🐾'}</span>
                      {pet.name}
                    </td>
                    <td className="px-4 py-3 text-slate-600">{pet.breed ?? '—'}</td>
                    <td className="px-4 py-3 text-slate-600">
                      {genderLabel(pet.gender)}
                    </td>
                    <td className="px-4 py-3 text-slate-600">
                      {pet.birthDate ?? '—'}
                      {pet.birthApproximate && (
                        <span className="ml-1 text-xs text-slate-400">（大概）</span>
                      )}
                    </td>
                    <td className="px-4 py-3 text-slate-600">
                      {neuterLabel(pet.neuterStatus)}
                    </td>
                    <td className="px-4 py-3 text-slate-600">{pet.companionDays}</td>
                    <td className="px-4 py-3 text-right">
                      <Link
                        to="/pet/$petId/edit"
                        params={{ petId: pet.petId }}
                        className="mr-2 text-slate-600 transition hover:text-slate-900"
                      >
                        编辑
                      </Link>
                      <button
                        type="button"
                        onClick={() => handleDelete(pet.petId)}
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

      {pendingDeleteId && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/40 px-4">
          <div className="w-full max-w-sm rounded-xl bg-white p-6 shadow-lg ring-1 ring-slate-200">
            <h2 className="text-lg font-semibold text-slate-900">删除宠物</h2>
            <p className="mt-2 text-sm text-slate-600">
              删除后宠物数据将归档保留 30 天。该操作仅监护人可执行，确认要继续吗？
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
                onClick={confirmDelete}
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

function genderLabel(gender: string): string {
  if (gender === 'male') return '公';
  if (gender === 'female') return '母';
  return gender;
}

function neuterLabel(status: string): string {
  if (status === 'neutered') return '已绝育';
  if (status === 'intact') return '未绝育';
  if (status === 'planned') return '计划中';
  return status;
}
