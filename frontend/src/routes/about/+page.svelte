<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<div class="min-h-full bg-gradient-to-br from-gray-50 to-gray-100 p-8">
	<div class="max-w-3xl mx-auto">
		<div class="bg-white border-4 border-black p-8 shadow-[8px_8px_0_#d06065]">
			<h1 class="text-4xl font-bold mb-6" style="font-family: 'VT323', monospace; color: #d06065;">About AEGIS</h1>
			
			<div class="space-y-4 text-lg">
				<p>
					<strong>AEGIS</strong> (Advanced Electronic Grievance Information System) is a campus-wide platform designed to give students and faculty a voice in addressing campus issues.
				</p>
				
				<p>
					Our mission is to create a transparent, efficient system for reporting, tracking, and resolving grievances that affect daily campus life.
				</p>
				
				<h2 class="text-2xl font-bold mt-8 mb-4" style="font-family: 'VT323', monospace;">Features</h2>
				<ul class="list-disc list-inside space-y-2 pl-4">
					<li>Submit grievances anonymously or with your identity</li>
					<li>Track the status of your submissions</li>
					<li>Upvote important issues to prioritize them</li>
					<li>Receive updates on resolution progress</li>
					<li>Engage with the campus administration transparently</li>
				</ul>
				
				<h2 class="text-2xl font-bold mt-8 mb-4" style="font-family: 'VT323', monospace;">Contact</h2>
				<p>
					For support or inquiries, please contact the campus administration or use the grievance system to report technical issues.
				</p>
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
