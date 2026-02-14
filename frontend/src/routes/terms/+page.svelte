<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<div class="min-h-full bg-gradient-to-br from-gray-50 to-gray-100 p-8">
	<div class="max-w-3xl mx-auto">
		<div class="bg-white border-4 border-black p-8 shadow-[8px_8px_0_#d06065]">
			<h1 class="text-4xl font-bold mb-6" style="font-family: 'VT323', monospace; color: #d06065;">Terms of Service</h1>
			
			<div class="space-y-6 text-base">
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">1. Acceptance of Terms</h2>
					<p>By accessing and using AEGIS, you accept and agree to be bound by these terms and conditions.</p>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">2. User Conduct</h2>
					<p>Users must:</p>
					<ul class="list-disc list-inside pl-4 space-y-1 mt-2">
						<li>Submit genuine grievances related to campus issues</li>
						<li>Refrain from abusive, defamatory, or harassing content</li>
						<li>Not impersonate others or submit false information</li>
						<li>Respect the privacy and rights of other users</li>
					</ul>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">3. Content Guidelines</h2>
					<p>All submissions must be appropriate and constructive. Content that violates campus policies or contains prohibited material will be removed.</p>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">4. Account Responsibility</h2>
					<p>Users are responsible for maintaining the confidentiality of their account credentials and for all activities under their account.</p>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">5. Modifications</h2>
					<p>We reserve the right to modify these terms at any time. Continued use of the service constitutes acceptance of modified terms.</p>
				</section>
			</div>
			
			<button 
				onclick={() => goto('/')} 
				class="mt-8 px-6 py-3 bg-[#ffb3ba] border-3 border-black font-bold hover:bg-[#d06065] hover:text-white transition-all"
				style="border: 3px solid black;"
			>
				← Back to Home
			</button>
		</div>
	</div>
</div>
