<script lang="ts">
	import { onMount } from 'svelte';
	import type { Snippet } from 'svelte';

	interface Page {
		id: string;
		title: string;
		icon: string;
		content: Snippet;
	}

	let {
		pages = [] as Page[],
		width = '600px',
		height = '800px',
		currentPage = $bindable(''),
		showBootSequence = true
	} = $props();

	let activeTab = $derived(currentPage || pages[0]?.id || 'status');

	function switchTab(pageId: string) {
		console.log('[PDA] switchTab called with:', pageId);
		console.log('[PDA] Current page before:', currentPage);
		currentPage = pageId;
		console.log('[PDA] Current page after:', currentPage);
	}

	$effect(() => {
		console.log('[PDA] activeTab changed to:', activeTab);
	});
</script>

<div class="pda2" style="--w: {width}; --h: {height};">
	<input type="radio" id="pda2TabStatus" class="pda2__tab" name="pdaTab" checked={activeTab === 'status'} />
	{#each pages as page, i}
		<input type="radio" id="pda2Tab{page.id}" class="pda2__tab" name="pdaTab" checked={activeTab === page.id} />
	{/each}
	<input type="checkbox" id="pda2__backlight" class="pda2__backlight" />

	<div class="pda2__bezel">
		<!-- Screws -->
		<div class="pda2__screws">
			<svg class="pda2__screw" viewBox="0 0 12 12">
				<circle cx="6" cy="6" r="5" fill="#1a1d22" />
				<line x1="3" y1="6" x2="9" y2="6" stroke="#0a0c0e" stroke-width="1.5" />
			</svg>
			<svg class="pda2__screw" viewBox="0 0 12 12">
				<circle cx="6" cy="6" r="5" fill="#1a1d22" />
				<line x1="3" y1="6" x2="9" y2="6" stroke="#0a0c0e" stroke-width="1.5" />
			</svg>
			<svg class="pda2__screw" viewBox="0 0 12 12">
				<circle cx="6" cy="6" r="5" fill="#1a1d22" />
				<line x1="3" y1="6" x2="9" y2="6" stroke="#0a0c0e" stroke-width="1.5" />
			</svg>
			<svg class="pda2__screw" viewBox="0 0 12 12">
				<circle cx="6" cy="6" r="5" fill="#1a1d22" />
				<line x1="3" y1="6" x2="9" y2="6" stroke="#0a0c0e" stroke-width="1.5" />
			</svg>
		</div>

		<!-- LED Indicator -->
		<div class="pda2__led"></div>

		<!-- Screen -->
		<div class="pda2__screen">
			<!-- Status Bar -->
			<div class="pda2__statusbar">
				<div class="pda2__logo">
					<span class="pda2__blink"></span>
				</div>
				<div class="pda2__ind">
					<div class="pda2__sig">
						<span class="pda2__sb pda2__sb--1"></span>
						<span class="pda2__sb pda2__sb--2"></span>
						<span class="pda2__sb pda2__sb--3"></span>
					</div>
					<div class="pda2__bat">
						<div class="pda2__bb"><div class="pda2__bf"></div></div>
						<span class="pda2__bt"></span>
					</div>
					<span class="pda2__clk">{new Date().toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>
				</div>
			</div>

			{#if showBootSequence}
			<!-- Boot Sequence -->
			<div class="pda2__boot">
				<div class="pda2__bline pda2__bline--1">AEGIS v2.1 INITIALIZING</div>
				<div class="pda2__bline pda2__bline--2">LOADING GRIEVANCE MOD</div>
				<div class="pda2__bline pda2__bline--3">CHECKING DATABASE...</div>
				<div class="pda2__bline pda2__bline--err">⚠ SYSTEM OPERATIONAL</div>
				<div class="pda2__bprog">
					<div class="pda2__bfill"></div>
				</div>
				<div class="pda2__bline pda2__bline--5">Press [ENTER] to continue</div>
			</div>
			{/if}

			<!-- Main Viewport -->
			<div class="pda2__viewport">
				<div class="pda2__apps" style="width: {pages.length * 100}%; grid-template-columns: repeat({pages.length}, 1fr); transform: translateX(-{pages.findIndex(p => p.id === activeTab) * (100 / pages.length)}%);">
					{#each pages as page}
						<div class="pda2__app">
							<div class="pda2__title">{page.title}</div>
							<div class="pda2__scroll">
							{@render page.content()}
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Dock -->
			<div class="pda2__dock">
				{#each pages as page}
					<button 
						class="pda2__db {activeTab === page.id ? 'pda2__db--active' : ''}" 
						onclick={() => switchTab(page.id)} 
						aria-label="Switch to {page.title}"
					>
						{@html page.icon}
					</button>
				{/each}
			</div>
		</div>

		<!-- Power Knob -->
		<label for="pda2__backlight" class="pda2__knob">
			<div class="pda2__knob-cap"></div>
		</label>
	</div>
</div>

<style>
.pda2 {
  --w: 600px;
  --h: 800px;

  --case-top: #676c73;
  --case-bot: #2b2f35;
  --lcd-bg: #f3d9a3;
  --lcd-ink: #3a2700;
  --lcd-dim: #a38241;
  --lcd-line: rgba(0, 0, 0, 0.06);
  --dock: #242b30;

  width: var(--w);
  height: var(--h);
  position: relative;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
  color: var(--lcd-ink);
}

.pda2__tab,
.pda2__backlight {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.pda2__bezel {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, var(--case-top), var(--case-bot));
  border-radius: 16px;
  box-shadow:
    inset 0 2px 0 rgba(255, 255, 255, 0.15),
    inset 0 -2px 0 rgba(0, 0, 0, 0.28),
    0 16px 28px rgba(0, 0, 0, 0.35);
  padding: 12px 10px 12px 10px;
}

.pda2__screws {
  position: absolute;
  inset: 6px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  pointer-events: none;
}
.pda2__screws .pda2__screw {
  width: 12px;
  height: 12px;
}
.pda2__screws .pda2__screw:nth-child(1) {
  justify-self: start;
  align-self: start;
}
.pda2__screws .pda2__screw:nth-child(2) {
  justify-self: end;
  align-self: start;
}
.pda2__screws .pda2__screw:nth-child(3) {
  justify-self: start;
  align-self: end;
}
.pda2__screws .pda2__screw:nth-child(4) {
  justify-self: end;
  align-self: end;
}

.pda2__knob {
  position: absolute;
  bottom: 10px;
  left: 50%;
  transform: translateX(-50%);
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: radial-gradient(circle at 35% 35%, #7b828a, #444a52);
  box-shadow:
    inset 0 3px 5px rgba(255, 255, 255, 0.18),
    inset 0 -3px 5px rgba(0, 0, 0, 0.5),
    0 6px 10px rgba(0, 0, 0, 0.3);
  cursor: pointer;
  user-select: none;
  display: grid;
  place-items: center;
}
.pda2__knob-cap {
  width: 8px;
  height: 8px;
  border-radius: 2px;
  background: #14181d;
  box-shadow:
    0 1px 0 rgba(255, 255, 255, 0.15),
    0 -1px 0 rgba(0, 0, 0, 0.6);
}

.pda2__led {
  position: absolute;
  top: 15px;
  left: 50%;
  transform: translateX(-50%);
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #4a1010;
  box-shadow:
    inset 0 0 2px #ff5b5b,
    0 0 0 1px #100;
}
.pda2__led::after {
  content: "";
  position: absolute;
  left: -2px;
  top: -2px;
  right: -2px;
  bottom: -2px;
  border-radius: 50%;
  box-shadow: 0 0 0 0 rgba(255, 70, 70, 0);
  pointer-events: none;
}

.pda2__screen {
  position: absolute;
  left: 10px;
  right: 10px;
  top: 40px;
  bottom: 48px;
  background: var(--lcd-bg);
  border-radius: 9px;
  overflow: hidden;
  box-shadow:
    inset 0 2px 0 rgba(255, 255, 255, 0.5),
    inset 0 -2px 0 rgba(0, 0, 0, 0.12),
    0 0 0 2px #101010;
}
.pda2__screen::before {
  content: "";
  position: absolute;
  inset: 0;
  background: repeating-linear-gradient(
      0deg,
      transparent 0 1px,
      var(--lcd-line) 1px 2px
    ),
    repeating-linear-gradient(
      90deg,
      transparent 0 1px,
      rgba(0, 0, 0, 0.03) 1px 2px
    );
  pointer-events: none;
  z-index: 100;
}
.pda2__screen::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(
      180deg,
      rgba(255, 255, 255, 0.12),
      transparent 45%
    ),
    radial-gradient(
      60% 40% at 10% 0%,
      rgba(255, 255, 255, 0.16),
      transparent 60%
    );
  mix-blend-mode: screen;
  pointer-events: none;
  z-index: 100;
}

.pda2__statusbar {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 20px;
  display: grid;
  grid-template-columns: 1fr auto;
  align-items: center;
  padding: 0 6px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.2), transparent);
  font-weight: 900;
  font-size: 11px;
  z-index: 2;
}
.pda2__logo {
  display: grid;
  grid-auto-flow: column;
  gap: 4px;
  align-items: center;
}
.pda2__blink {
  width: 5px;
  height: 10px;
  background: var(--lcd-ink);
  display: inline-block;
  animation: pda2_blink 1s steps(2, end) infinite;
}
@keyframes pda2_blink {
  50% { opacity: 0; }
}
.pda2__ind {
  display: grid;
  grid-auto-flow: column;
  gap: 6px;
  align-items: center;
}
.pda2__sig {
  display: grid;
  grid-auto-flow: column;
  gap: 2px;
}
.pda2__sb {
  width: 3px;
  background: var(--lcd-ink);
  display: inline-block;
}
.pda2__sb--1 { height: 5px; }
.pda2__sb--2 { height: 8px; }
.pda2__sb--3 { height: 10px; }
.pda2__bat {
  display: grid;
  grid-auto-flow: column;
  gap: 2px;
  align-items: center;
}
.pda2__bb {
  width: 16px;
  height: 9px;
  border: 1px solid var(--lcd-ink);
  position: relative;
}
.pda2__bf {
  position: absolute;
  left: 1px;
  top: 1px;
  bottom: 1px;
  width: 10px;
  background: var(--lcd-ink);
  animation: pda2_bat 3s steps(3, end) infinite;
}
@keyframes pda2_bat {
  0%, 100% { width: 10px; }
  50% { width: 14px; }
}
.pda2__bt {
  width: 2px;
  height: 5px;
  background: var(--lcd-ink);
}
.pda2__clk {
  font-weight: 900;
}

.pda2__boot {
  position: absolute;
  inset: 20px 0 26px 0;
  padding: 8px 8px 0;
  background: linear-gradient(
    180deg,
    rgba(255, 255, 255, 0.25),
    rgba(255, 255, 255, 0.12)
  );
  animation: pda2_boot_fade 2.7s 0.2s ease forwards;
  z-index: 3;
}
@keyframes pda2_boot_fade {
  0% { opacity: 1; }
  85% { opacity: 1; }
  100% { opacity: 0; pointer-events: none; }
}
.pda2__bline {
  color: var(--lcd-ink);
  font-weight: 900;
  font-size: 11px;
  white-space: nowrap;
  overflow: hidden;
}
.pda2__bline--1 {
  max-width: 0;
  animation: pda2_type 0.85s steps(22, end) forwards;
}
.pda2__bline--2 {
  max-width: 0;
  animation: pda2_type 0.6s 0.85s steps(20, end) forwards;
}
.pda2__bline--3 {
  max-width: 0;
  animation: pda2_type 0.6s 1.45s steps(18, end) forwards;
}
.pda2__bline--err {
  margin: 2px 0;
  padding: 2px 4px;
  border: 1px dashed var(--lcd-ink);
  background: rgba(0, 0, 0, 0.06);
  animation: pda2_errflash 0.9s 2.05s steps(2, end) 2;
}
@keyframes pda2_errflash {
  50% { filter: invert(1); }
}
.pda2__bprog {
  margin-top: 2px;
  height: 7px;
  border: 1px solid var(--lcd-ink);
  position: relative;
}
.pda2__bfill {
  position: absolute;
  inset: 1px;
  width: 0;
  background: var(--lcd-ink);
  animation: pda2_fill 1.1s 2.05s steps(12, end) forwards;
}
@keyframes pda2_fill {
  to { width: calc(100% - 2px); }
}
.pda2__bline--5 {
  position: absolute;
  bottom: 2px;
  left: 8px;
  opacity: 0.95;
}
@keyframes pda2_type {
  to { max-width: 100%; }
}

.pda2__viewport {
  position: absolute;
  inset: 20px 0 26px 0;
  overflow: hidden;
}
.pda2__apps {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  display: grid;
  transition: transform 0.48s cubic-bezier(0.2, 0.7, 0.2, 1);
}

.pda2__app {
  position: relative;
  padding: 6px;
  display: grid;
  grid-template-rows: auto 1fr;
  gap: 6px;
  background:
    radial-gradient(3px 3px at 22% 32%, rgba(0, 0, 0, 0.03) 35%, transparent 40%) 0 0/10px 10px,
    radial-gradient(3px 3px at 70% 60%, rgba(0, 0, 0, 0.03) 35%, transparent 40%) 0 0/12px 12px,
    transparent;
}
.pda2__title {
  font-weight: 900;
  font-size: 11px;
  text-transform: uppercase;
  border-bottom: 1px dashed rgba(0, 0, 0, 0.15);
  padding-bottom: 3px;
}

.pda2__scroll {
  overflow: hidden;
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.pda2__scroll::-webkit-scrollbar {
  display: none;
}

.pda2__dock {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  height: 26px;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 0;
  background: linear-gradient(180deg, rgba(0, 0, 0, 0.25), rgba(0, 0, 0, 0.5));
  border-top: 1px solid #0f0f0f;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.1);
  z-index: 2;
}
.pda2__db {
  display: grid;
  place-items: center;
  cursor: pointer;
  transition: filter 0.15s;
  position: relative;
	background: transparent;
	border: none;
	padding: 0;
}
.pda2__db:hover {
	filter: brightness(1.1);
}
.pda2__db--active::before {
	content: '';
	position: absolute;
	bottom: 2px;
	left: 50%;
	transform: translateX(-50%);
	width: 50%;
	height: 2px;
	background: var(--lcd-ink);
	opacity: 0.8;
}

.pda2__backlight {
	position: absolute;
	opacity: 0;
	pointer-events: none;
}

.pda2__backlight:checked ~ .pda2__bezel .pda2__screen {
	filter: brightness(1.12) contrast(1.05) saturate(1.06);
}

.pda2__backlight:not(:checked) ~ .pda2__bezel .pda2__screen {
	filter: brightness(0.88) contrast(0.95) saturate(0.92);
}
</style>
