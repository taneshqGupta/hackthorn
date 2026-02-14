<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<div class="min-h-full bg-gradient-to-br from-gray-50 to-gray-100 p-8">
	<div class="max-w-3xl mx-auto">
		<div class="bg-white border-4 border-black p-8 shadow-[8px_8px_0_#d06065]">
			<h1 class="text-4xl font-bold mb-6" style="font-family: 'VT323', monospace; color: #d06065;">Privacy Policy</h1>
			
			<div class="space-y-6 text-base">
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">1. Information We Collect</h2>
					<p>We collect information you provide when:</p>
					<ul class="list-disc list-inside pl-4 space-y-1 mt-2">
						<li>Creating an account (name, email, profile picture via Google OAuth)</li>
						<li>Submitting grievances (title, description, category, attachments)</li>
						<li>Interacting with the platform (comments, upvotes)</li>
					</ul>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">2. Anonymous Submissions</h2>
					<p>When you submit a grievance anonymously, your identity is not stored with the grievance. However, system logs may retain your IP address for security purposes.</p>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">3. How We Use Your Information</h2>
					<p>Your information is used to:</p>
					<ul class="list-disc list-inside pl-4 space-y-1 mt-2">
						<li>Manage and track grievances</li>
						<li>Communicate updates and resolutions</li>
						<li>Improve the platform and user experience</li>
						<li>Ensure security and prevent abuse</li>
					</ul>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">4. Data Security</h2>
					<p>We implement appropriate security measures to protect your personal information from unauthorized access, alteration, or disclosure.</p>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">5. Data Sharing</h2>
					<p>We do not sell or share your personal information with third parties, except as required by law or to resolve reported issues with relevant campus authorities.</p>
				</section>
				
				<section>
					<h2 class="text-2xl font-bold mb-3" style="font-family: 'VT323', monospace;">6. Your Rights</h2>
					<p>You have the right to access, update, or delete your personal information. Contact us through the platform for assistance.</p>
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
