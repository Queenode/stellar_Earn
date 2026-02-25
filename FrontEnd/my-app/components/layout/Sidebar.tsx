"use client";

import { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { LevelBadge } from '@/components/reputation/LevelBadge';
import OptimizedImage from '@/components/ui/OptimizedImage';


interface SidebarProps {
  collapsed?: boolean;
}

function NavIcon({ active }: { active: boolean }) {
  return (
    <span
      aria-hidden="true"
      className={`inline-block h-2.5 w-2.5 rounded-full ${active ? "bg-white" : "bg-zinc-400"}`}
    />
  );
}

export function Sidebar({ collapsed = false }: SidebarProps) {
  const pathname = usePathname();

  return (
    <aside
      aria-label="Sidebar navigation"
      className={`hidden border-r border-zinc-200 bg-white transition-[width] duration-300 dark:border-zinc-800 dark:bg-zinc-950 lg:flex lg:flex-col ${
        collapsed ? "lg:w-20" : "lg:w-72"
      }`}
    >
      <div
        className={`fixed left-0 top-0 z-40 flex h-screen w-64 flex-col border-r border-zinc-200 bg-white transition-transform dark:border-zinc-800 dark:bg-zinc-900 lg:relative lg:translate-x-0 ${isMobileOpen ? 'translate-x-0' : '-translate-x-full'
          }`}
      >
        {/* Logo */}
        <div className="flex items-center gap-2 border-b border-zinc-200 px-6 py-4 dark:border-zinc-800">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg text-white" style={{ backgroundColor: '#089ec3' }}>
            <svg className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <span className="text-lg font-semibold text-zinc-900 dark:text-zinc-50">
            StellarEarn
          </span>
        </div>

        {/* Navigation */}
        <nav className="flex-1 space-y-1 px-3 py-4">
          {navigation.map((item) => {
            const isActive = pathname === item.href;
            const Icon = item.icon;
            return (
              <Link
                key={item.name}
                href={item.href}
                onClick={() => setIsMobileOpen(false)}
                className={`flex items-center gap-3 rounded-lg px-4 py-3 text-base font-medium transition-colors ${isActive
                  ? 'text-white'
                  : 'text-zinc-700 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800'
                  }`}
                style={isActive ? { backgroundColor: '#089ec3' } : {}}
              >
                <Icon />
                {item.name}
              </Link>
            );
          })}
        </nav>

        {/* User Profile */}
        <div className="border-t border-zinc-200 px-6 py-4 dark:border-zinc-800">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-full text-sm font-medium text-white" style={{ backgroundColor: '#089ec3' }}>
              {user.avatar ? (
                <OptimizedImage
                  src={user.avatar}
                  alt={user.name}
                  width={40}
                  height={40}
                  containerClassName="h-full w-full rounded-full"
                  className="h-full w-full rounded-full"
                />
              ) : (
                <span>{user.name.split('.').map(n => n[0]).join('').toUpperCase()}</span>
              )}
            </div>
            <div className="flex-1 min-w-0">
              <p className="text-sm font-medium text-zinc-900 dark:text-zinc-50 truncate">
                {user.name}
              </p>
              <div className="flex items-center gap-2 mt-1">
                <LevelBadge level={user.level} size="sm" />
                <span className="text-xs text-zinc-500 dark:text-zinc-400">
                  Level {user.level}
                </span>
              </div>
            </div>
          </div>
          <Link
            href="/signout"
            className="mt-4 flex items-center gap-2 text-sm text-zinc-600 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-50"
          >
            <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
            </svg>
            Sign Out
          </Link>
        </div>
      </div>
    </>
  );
}
