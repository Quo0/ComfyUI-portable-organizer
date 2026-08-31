# Workflow compatibility

<!-- US-WF-05 -->

The reason moving workflows lives in the app rather than in Explorer.

Before adding one, you can see whether the target build has all the nodes it
needs. There are three states, not two: a stopped instance has no exact answer,
and marking it **unknown** is more honest than showing a green tick
`US-WF-05/AC-6`. Missing nodes warn but do not forbid — they can be installed
afterwards `AC-4`.

<ThemePair>
  <div class="compat">
    <div class="compat-row ok">
      <span class="chip" style="--instance-accent:var(--accent-teal)"></span>
      <span>SDXL stable</span>
      <span class="compat-note">all nodes present</span>
    </div>
    <div class="compat-row warn">
      <span class="chip" style="--instance-accent:var(--accent-indigo)"></span>
      <span>Flux test</span>
      <span class="compat-note">2 nodes missing</span>
    </div>
    <div class="missing">IPAdapterUnifiedLoader · ReActorFaceSwap</div>
    <div class="compat-row">
      <span class="chip" style="--instance-accent:var(--accent-moss)"></span>
      <span>Experiments</span>
      <span class="compat-note">per the last run</span>
    </div>
  </div>
</ThemePair>
