<html>

<head>
    <style>
        @media (prefers-color-scheme: dark) {
            body {
                background: #333;
                color: white;
            }
            a {
                color: skyblue;
            }
        }
        .container {
            font-family: sans-serif;
            max-width: 800px;
            margin: 0 auto;
        }
        .intro {
            text-align: center;
        }
        .licenses-list {
            list-style-type: none;
            margin: 0;
            padding: 0;
        }
        .license-used-by {
            margin-top: -10px;
        }
        .license-text {
            max-height: 200px;
            overflow-y: scroll;
            white-space: pre-wrap;
        }
    </style>
</head>

<body>
    <main class="container">
        <div class="intro">
            <h1>Third Party Licenses</h1>
            <p>This page lists the licenses of the projects used in cargo-about.</p>
        </div>
    
        <h2>Overview of licenses:</h2>
        <ul class="licenses-overview">
            <li><a href="#MIT">MIT License</a> (323)</li>
            <li><a href="#Unicode-3.0">Unicode License v3</a> (19)</li>
            <li><a href="#Apache-2.0">Apache License 2.0</a> (12)</li>
            <li><a href="#BSD-3-Clause">BSD 3-Clause &quot;New&quot; or &quot;Revised&quot; License</a> (3)</li>
            <li><a href="#BSL-1.0">Boost Software License 1.0</a> (2)</li>
            <li><a href="#ISC">ISC License</a> (2)</li>
            <li><a href="#BSD-2-Clause">BSD 2-Clause &quot;Simplified&quot; License</a> (1)</li>
            <li><a href="#CECILL-2.1">CeCILL Free Software License Agreement v2.1</a> (1)</li>
            <li><a href="#OFL-1.1">SIL Open Font License 1.1</a> (1)</li>
            <li><a href="#Zlib">zlib License</a> (1)</li>
        </ul>

        <h2>All license text:</h2>
        <ul class="licenses-list">
            <li class="license">
                <h3 id="Apache-2.0">Apache License 2.0</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://codeberg.org/swsnr/gethostname.rs.git ">gethostname 1.0.2</a></li>
                </ul>
                <pre class="license-text">                              Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   &quot;License&quot; shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   &quot;Licensor&quot; shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   &quot;Legal Entity&quot; shall mean the union of the acting entity and all
   other entities that control, are controlled by, or are under common
   control with that entity. For the purposes of this definition,
   &quot;control&quot; means (i) the power, direct or indirect, to cause the
   direction or management of such entity, whether by contract or
   otherwise, or (ii) ownership of fifty percent (50%) or more of the
   outstanding shares, or (iii) beneficial ownership of such entity.

   &quot;You&quot; (or &quot;Your&quot;) shall mean an individual or Legal Entity
   exercising permissions granted by this License.

   &quot;Source&quot; form shall mean the preferred form for making modifications,
   including but not limited to software source code, documentation
   source, and configuration files.

   &quot;Object&quot; form shall mean any form resulting from mechanical
   transformation or translation of a Source form, including but
   not limited to compiled object code, generated documentation,
   and conversions to other media types.

   &quot;Work&quot; shall mean the work of authorship, whether in Source or
   Object form, made available under the License, as indicated by a
   copyright notice that is included in or attached to the work
   (an example is provided in the Appendix below).

   &quot;Derivative Works&quot; shall mean any work, whether in Source or Object
   form, that is based on (or derived from) the Work and for which the
   editorial revisions, annotations, elaborations, or other modifications
   represent, as a whole, an original work of authorship. For the purposes
   of this License, Derivative Works shall not include works that remain
   separable from, or merely link (or bind by name) to the interfaces of,
   the Work and Derivative Works thereof.

   &quot;Contribution&quot; shall mean any work of authorship, including
   the original version of the Work and any modifications or additions
   to that Work or Derivative Works thereof, that is intentionally
   submitted to Licensor for inclusion in the Work by the copyright owner
   or by an individual or Legal Entity authorized to submit on behalf of
   the copyright owner. For the purposes of this definition, &quot;submitted&quot;
   means any form of electronic, verbal, or written communication sent
   to the Licensor or its representatives, including but not limited to
   communication on electronic mailing lists, source code control systems,
   and issue tracking systems that are managed by, or on behalf of, the
   Licensor for the purpose of discussing and improving the Work, but
   excluding communication that is conspicuously marked or otherwise
   designated in writing by the copyright owner as &quot;Not a Contribution.&quot;

   &quot;Contributor&quot; shall mean Licensor and any individual or Legal Entity
   on behalf of whom a Contribution has been received by Licensor and
   subsequently incorporated within the Work.

2. Grant of Copyright License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   copyright license to reproduce, prepare Derivative Works of,
   publicly display, publicly perform, sublicense, and distribute the
   Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of
   this License, each Contributor hereby grants to You a perpetual,
   worldwide, non-exclusive, no-charge, royalty-free, irrevocable
   (except as stated in this section) patent license to make, have made,
   use, offer to sell, sell, import, and otherwise transfer the Work,
   where such license applies only to those patent claims licensable
   by such Contributor that are necessarily infringed by their
   Contribution(s) alone or by combination of their Contribution(s)
   with the Work to which such Contribution(s) was submitted. If You
   institute patent litigation against any entity (including a
   cross-claim or counterclaim in a lawsuit) alleging that the Work
   or a Contribution incorporated within the Work constitutes direct
   or contributory patent infringement, then any patent licenses
   granted to You under this License for that Work shall terminate
   as of the date such litigation is filed.

4. Redistribution. You may reproduce and distribute copies of the
   Work or Derivative Works thereof in any medium, with or without
   modifications, and in Source or Object form, provided that You
   meet the following conditions:

   (a) You must give any other recipients of the Work or
       Derivative Works a copy of this License; and

   (b) You must cause any modified files to carry prominent notices
       stating that You changed the files; and

   (c) You must retain, in the Source form of any Derivative Works
       that You distribute, all copyright, patent, trademark, and
       attribution notices from the Source form of the Work,
       excluding those notices that do not pertain to any part of
       the Derivative Works; and

   (d) If the Work includes a &quot;NOTICE&quot; text file as part of its
       distribution, then any Derivative Works that You distribute must
       include a readable copy of the attribution notices contained
       within such NOTICE file, excluding those notices that do not
       pertain to any part of the Derivative Works, in at least one
       of the following places: within a NOTICE text file distributed
       as part of the Derivative Works; within the Source form or
       documentation, if provided along with the Derivative Works; or,
       within a display generated by the Derivative Works, if and
       wherever such third-party notices normally appear. The contents
       of the NOTICE file are for informational purposes only and
       do not modify the License. You may add Your own attribution
       notices within Derivative Works that You distribute, alongside
       or as an addendum to the NOTICE text from the Work, provided
       that such additional attribution notices cannot be construed
       as modifying the License.

   You may add Your own copyright statement to Your modifications and
   may provide additional or different license terms and conditions
   for use, reproduction, or distribution of Your modifications, or
   for any such Derivative Works as a whole, provided Your use,
   reproduction, and distribution of the Work otherwise complies with
   the conditions stated in this License.

5. Submission of Contributions. Unless You explicitly state otherwise,
   any Contribution intentionally submitted for inclusion in the Work
   by You to the Licensor shall be under the terms and conditions of
   this License, without any additional terms or conditions.
   Notwithstanding the above, nothing herein shall supersede or modify
   the terms of any separate license agreement you may have executed
   with Licensor regarding such Contributions.

6. Trademarks. This License does not grant permission to use the trade
   names, trademarks, service marks, or product names of the Licensor,
   except as required for reasonable and customary use in describing the
   origin of the Work and reproducing the content of the NOTICE file.

7. Disclaimer of Warranty. Unless required by applicable law or
   agreed to in writing, Licensor provides the Work (and each
   Contributor provides its Contributions) on an &quot;AS IS&quot; BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
   implied, including, without limitation, any warranties or conditions
   of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
   PARTICULAR PURPOSE. You are solely responsible for determining the
   appropriateness of using or redistributing the Work and assume any
   risks associated with Your exercise of permissions under this License.

8. Limitation of Liability. In no event and under no legal theory,
   whether in tort (including negligence), contract, or otherwise,
   unless required by applicable law (such as deliberate and grossly
   negligent acts) or agreed to in writing, shall any Contributor be
   liable to You for damages, including any direct, indirect, special,
   incidental, or consequential damages of any character arising as a
   result of this License or out of the use or inability to use the
   Work (including but not limited to damages for loss of goodwill,
   work stoppage, computer failure or malfunction, or any and all
   other commercial damages or losses), even if such Contributor
   has been advised of the possibility of such damages.

9. Accepting Warranty or Additional Liability. While redistributing
   the Work or Derivative Works thereof, You may choose to offer,
   and charge a fee for, acceptance of support, warranty, indemnity,
   or other liability obligations and/or rights consistent with this
   License. However, in accepting such obligations, You may act only
   on Your own behalf and on Your sole responsibility, not on behalf
   of any other Contributor, and only if You agree to indemnify,
   defend, and hold each Contributor harmless for any liability
   incurred by, or claims asserted against, such Contributor by reason
   of your accepting any such warranty or additional liability.

END OF TERMS AND CONDITIONS

APPENDIX: How to apply the Apache License to your work.

   To apply the Apache License to your work, attach the following
   boilerplate notice, with the fields enclosed by brackets &quot;[]&quot;
   replaced with your own identifying information. (Don&#x27;t include
   the brackets!)  The text should be enclosed in the appropriate
   comment syntax for the file format. We also recommend that a
   file or class name and description of purpose be included on the
   same &quot;printed page&quot; as the copyright notice for easier
   identification within third-party archives.

Copyright [yyyy] [name of copyright owner]

Licensed under the Apache License, Version 2.0 (the &quot;License&quot;);
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an &quot;AS IS&quot; BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
</pre>
            </li>
            <li class="license">
                <h3 id="Apache-2.0">Apache License 2.0</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-windowing/glutin ">glutin 0.30.10</a></li>
                    <li><a href=" https://github.com/rust-windowing/glutin ">glutin_egl_sys 0.5.1</a></li>
                    <li><a href=" https://github.com/rust-windowing/glutin ">glutin_glx_sys 0.4.0</a></li>
                    <li><a href=" https://github.com/rust-windowing/glutin ">glutin_wgl_sys 0.4.0</a></li>
                </ul>
                <pre class="license-text">Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      &quot;License&quot; shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      &quot;Licensor&quot; shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      &quot;Legal Entity&quot; shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      &quot;control&quot; means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      &quot;You&quot; (or &quot;Your&quot;) shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      &quot;Source&quot; form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      &quot;Object&quot; form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      &quot;Work&quot; shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      &quot;Derivative Works&quot; shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      &quot;Contribution&quot; shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, &quot;submitted&quot;
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as &quot;Not a Contribution.&quot;

      &quot;Contributor&quot; shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a &quot;NOTICE&quot; text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an &quot;AS IS&quot; BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets &quot;{}&quot;
      replaced with your own identifying information. (Don&#x27;t include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same &quot;printed page&quot; as the copyright notice for easier
      identification within third-party archives.

   Copyright 2022 Kirill Chibisov

   Licensed under the Apache License, Version 2.0 (the &quot;License&quot;);
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an &quot;AS IS&quot; BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
</pre>
            </li>
            <li class="license">
                <h3 id="Apache-2.0">Apache License 2.0</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-windowing/winit ">winit 0.28.7</a></li>
                </ul>
                <pre class="license-text">Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      &quot;License&quot; shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      &quot;Licensor&quot; shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      &quot;Legal Entity&quot; shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      &quot;control&quot; means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      &quot;You&quot; (or &quot;Your&quot;) shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      &quot;Source&quot; form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      &quot;Object&quot; form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      &quot;Work&quot; shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      &quot;Derivative Works&quot; shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      &quot;Contribution&quot; shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, &quot;submitted&quot;
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as &quot;Not a Contribution.&quot;

      &quot;Contributor&quot; shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a &quot;NOTICE&quot; text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an &quot;AS IS&quot; BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets &quot;{}&quot;
      replaced with your own identifying information. (Don&#x27;t include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same &quot;printed page&quot; as the copyright notice for easier
      identification within third-party archives.

   Copyright {yyyy} {name of copyright owner}

   Licensed under the Apache License, Version 2.0 (the &quot;License&quot;);
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an &quot;AS IS&quot; BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.</pre>
            </li>
            <li class="license">
                <h3 id="Apache-2.0">Apache License 2.0</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/alexheretic/ab-glyph ">ab_glyph 0.2.31</a></li>
                    <li><a href=" https://github.com/alexheretic/ab-glyph ">ab_glyph_rasterizer 0.1.10</a></li>
                    <li><a href=" https://github.com/AccessKit/accesskit ">accesskit_winit 0.15.0</a></li>
                    <li><a href=" https://github.com/brendanzab/gl-rs/ ">gl_generator 0.14.0</a></li>
                    <li><a href=" https://github.com/brendanzab/gl-rs/ ">khronos_api 3.1.0</a></li>
                    <li><a href=" https://github.com/alexheretic/owned-ttf-parser ">owned_ttf_parser 0.25.1</a></li>
                </ul>
                <pre class="license-text">Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

&quot;License&quot; shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.

&quot;Licensor&quot; shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.

&quot;Legal Entity&quot; shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity. For the purposes of this definition, &quot;control&quot; means (i) the power, direct or indirect, to cause the direction or management of such entity, whether by contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the outstanding shares, or (iii) beneficial ownership of such entity.

&quot;You&quot; (or &quot;Your&quot;) shall mean an individual or Legal Entity exercising permissions granted by this License.

&quot;Source&quot; form shall mean the preferred form for making modifications, including but not limited to software source code, documentation source, and configuration files.

&quot;Object&quot; form shall mean any form resulting from mechanical transformation or translation of a Source form, including but not limited to compiled object code, generated documentation, and conversions to other media types.

&quot;Work&quot; shall mean the work of authorship, whether in Source or Object form, made available under the License, as indicated by a copyright notice that is included in or attached to the work (an example is provided in the Appendix below).

&quot;Derivative Works&quot; shall mean any work, whether in Source or Object form, that is based on (or derived from) the Work and for which the editorial revisions, annotations, elaborations, or other modifications represent, as a whole, an original work of authorship. For the purposes of this License, Derivative Works shall not include works that remain separable from, or merely link (or bind by name) to the interfaces of, the Work and Derivative Works thereof.

&quot;Contribution&quot; shall mean any work of authorship, including the original version of the Work and any modifications or additions to that Work or Derivative Works thereof, that is intentionally submitted to Licensor for inclusion in the Work by the copyright owner or by an individual or Legal Entity authorized to submit on behalf of the copyright owner. For the purposes of this definition, &quot;submitted&quot; means any form of electronic, verbal, or written communication sent to the Licensor or its representatives, including but not limited to communication on electronic mailing lists, source code control systems, and issue tracking systems that are managed by, or on behalf of, the Licensor for the purpose of discussing and improving the Work, but excluding communication that is conspicuously marked or otherwise designated in writing by the copyright owner as &quot;Not a Contribution.&quot;

&quot;Contributor&quot; shall mean Licensor and any individual or Legal Entity on behalf of whom a Contribution has been received by Licensor and subsequently incorporated within the Work.

2. Grant of Copyright License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license to reproduce, prepare Derivative Works of, publicly display, publicly perform, sublicense, and distribute the Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable (except as stated in this section) patent license to make, have made, use, offer to sell, sell, import, and otherwise transfer the Work, where such license applies only to those patent claims licensable by such Contributor that are necessarily infringed by their Contribution(s) alone or by combination of their Contribution(s) with the Work to which such Contribution(s) was submitted. If You institute patent litigation against any entity (including a cross-claim or counterclaim in a lawsuit) alleging that the Work or a Contribution incorporated within the Work constitutes direct or contributory patent infringement, then any patent licenses granted to You under this License for that Work shall terminate as of the date such litigation is filed.

4. Redistribution. You may reproduce and distribute copies of the Work or Derivative Works thereof in any medium, with or without modifications, and in Source or Object form, provided that You meet the following conditions:

     (a) You must give any other recipients of the Work or Derivative Works a copy of this License; and

     (b) You must cause any modified files to carry prominent notices stating that You changed the files; and

     (c) You must retain, in the Source form of any Derivative Works that You distribute, all copyright, patent, trademark, and attribution notices from the Source form of the Work, excluding those notices that do not pertain to any part of the Derivative Works; and

     (d) If the Work includes a &quot;NOTICE&quot; text file as part of its distribution, then any Derivative Works that You distribute must include a readable copy of the attribution notices contained within such NOTICE file, excluding those notices that do not pertain to any part of the Derivative Works, in at least one of the following places: within a NOTICE text file distributed as part of the Derivative Works; within the Source form or documentation, if provided along with the Derivative Works; or, within a display generated by the Derivative Works, if and wherever such third-party notices normally appear. The contents of the NOTICE file are for informational purposes only and do not modify the License. You may add Your own attribution notices within Derivative Works that You distribute, alongside or as an addendum to the NOTICE text from the Work, provided that such additional attribution notices cannot be construed as modifying the License.

     You may add Your own copyright statement to Your modifications and may provide additional or different license terms and conditions for use, reproduction, or distribution of Your modifications, or for any such Derivative Works as a whole, provided Your use, reproduction, and distribution of the Work otherwise complies with the conditions stated in this License.

5. Submission of Contributions. Unless You explicitly state otherwise, any Contribution intentionally submitted for inclusion in the Work by You to the Licensor shall be under the terms and conditions of this License, without any additional terms or conditions. Notwithstanding the above, nothing herein shall supersede or modify the terms of any separate license agreement you may have executed with Licensor regarding such Contributions.

6. Trademarks. This License does not grant permission to use the trade names, trademarks, service marks, or product names of the Licensor, except as required for reasonable and customary use in describing the origin of the Work and reproducing the content of the NOTICE file.

7. Disclaimer of Warranty. Unless required by applicable law or agreed to in writing, Licensor provides the Work (and each Contributor provides its Contributions) on an &quot;AS IS&quot; BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied, including, without limitation, any warranties or conditions of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A PARTICULAR PURPOSE. You are solely responsible for determining the appropriateness of using or redistributing the Work and assume any risks associated with Your exercise of permissions under this License.

8. Limitation of Liability. In no event and under no legal theory, whether in tort (including negligence), contract, or otherwise, unless required by applicable law (such as deliberate and grossly negligent acts) or agreed to in writing, shall any Contributor be liable to You for damages, including any direct, indirect, special, incidental, or consequential damages of any character arising as a result of this License or out of the use or inability to use the Work (including but not limited to damages for loss of goodwill, work stoppage, computer failure or malfunction, or any and all other commercial damages or losses), even if such Contributor has been advised of the possibility of such damages.

9. Accepting Warranty or Additional Liability. While redistributing the Work or Derivative Works thereof, You may choose to offer, and charge a fee for, acceptance of support, warranty, indemnity, or other liability obligations and/or rights consistent with this License. However, in accepting such obligations, You may act only on Your own behalf and on Your sole responsibility, not on behalf of any other Contributor, and only if You agree to indemnify, defend, and hold each Contributor harmless for any liability incurred by, or claims asserted against, such Contributor by reason of your accepting any such warranty or additional liability.

END OF TERMS AND CONDITIONS

APPENDIX: How to apply the Apache License to your work.

To apply the Apache License to your work, attach the following boilerplate notice, with the fields enclosed by brackets &quot;[]&quot; replaced with your own identifying information. (Don&#x27;t include the brackets!)  The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same &quot;printed page&quot; as the copyright notice for easier identification within third-party archives.

Copyright [yyyy] [name of copyright owner]

Licensed under the Apache License, Version 2.0 (the &quot;License&quot;);
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an &quot;AS IS&quot; BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
</pre>
            </li>
            <li class="license">
                <h3 id="BSD-2-Clause">BSD 2-Clause &quot;Simplified&quot; License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/droundy/arrayref ">arrayref 0.3.9</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2015 David Roundy &lt;roundyd@physics.oregonstate.edu&gt;
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the
   distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
&quot;AS IS&quot; AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
</pre>
            </li>
            <li class="license">
                <h3 id="BSD-3-Clause">BSD 3-Clause &quot;New&quot; or &quot;Revised&quot; License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RazrFalcon/tiny-skia/path ">tiny-skia-path 0.8.4</a></li>
                    <li><a href=" https://github.com/RazrFalcon/tiny-skia ">tiny-skia 0.8.4</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2011 Google Inc. All rights reserved.
Copyright (c) 2020 Yevhenii Reizner All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

  * Redistributions of source code must retain the above copyright
    notice, this list of conditions and the following disclaimer.

  * Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in
    the documentation and/or other materials provided with the
    distribution.

  * Neither the name of the copyright holder nor the names of its
    contributors may be used to endorse or promote products derived
    from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
&quot;AS IS&quot; AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
</pre>
            </li>
            <li class="license">
                <h3 id="BSD-3-Clause">BSD 3-Clause &quot;New&quot; or &quot;Revised&quot; License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/sebcrozet/instant ">instant 0.1.13</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2019, Sébastien Crozet
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the author nor the names of its contributors may be used
   to endorse or promote products derived from this software without specific
   prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS &quot;AS IS&quot; AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
</pre>
            </li>
            <li class="license">
                <h3 id="BSL-1.0">Boost Software License 1.0</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/DoumanAsh/error-code ">error-code 3.3.2</a></li>
                </ul>
                <pre class="license-text">Boost Software License - Version 1.0 - August 17th, 2003

Permission is hereby granted, free of charge, to any person or organization
obtaining a copy of the software and accompanying documentation covered by
this license (the &quot;Software&quot;) to use, reproduce, display, distribute,
execute, and transmit the Software, and to prepare derivative works of the
Software, and to permit third-parties to whom the Software is furnished to
do so, all subject to the following:

The copyright notices in the Software and this entire statement, including
the above license grant, this restriction and the following disclaimer,
must be included in all copies of the Software, in whole or in part, and
all derivative works of the Software, unless such copies or derivative
works are solely in the form of machine-executable object code generated by
a source language processor.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT
SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE
FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="BSL-1.0">Boost Software License 1.0</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/DoumanAsh/clipboard-win ">clipboard-win 5.4.1</a></li>
                </ul>
                <pre class="license-text">Boost Software License - Version 1.0 - August 17th, 2003

Permission is hereby granted, free of charge, to any person or organization obtaining a copy of the software and accompanying documentation covered by this license (the &quot;Software&quot;) to use, reproduce, display, distribute, execute, and transmit the Software, and to prepare derivative works of the Software, and to permit third-parties to whom the Software is furnished to do so, all subject to the following:

The copyright notices in the Software and this entire statement, including the above license grant, this restriction and the following disclaimer, must be included in all copies of the Software, in whole or in part, and all derivative works of the Software, unless such copies or derivative works are solely in the form of machine-executable object code generated by a source language processor.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="CECILL-2.1">CeCILL Free Software License Agreement v2.1</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://crates.io/crates/GazBox ">GazBox 0.0.1</a></li>
                </ul>
                <pre class="license-text">
  CeCILL FREE SOFTWARE LICENSE AGREEMENT

Version 2.1 dated 2013-06-21


    Notice

This Agreement is a Free Software license agreement that is the result
of discussions between its authors in order to ensure compliance with
the two main principles guiding its drafting:

  * firstly, compliance with the principles governing the distribution
    of Free Software: access to source code, broad rights granted to users,
  * secondly, the election of a governing law, French law, with which it
    is conformant, both as regards the law of torts and intellectual
    property law, and the protection that it offers to both authors and
    holders of the economic rights over software.

The authors of the CeCILL (for Ce[a] C[nrs] I[nria] L[ogiciel] L[ibre])
license are:

Commissariat à l&#x27;énergie atomique et aux énergies alternatives - CEA, a
public scientific, technical and industrial research establishment,
having its principal place of business at 25 rue Leblanc, immeuble Le
Ponant D, 75015 Paris, France.

Centre National de la Recherche Scientifique - CNRS, a public scientific
and technological establishment, having its principal place of business
at 3 rue Michel-Ange, 75794 Paris cedex 16, France.

Institut National de Recherche en Informatique et en Automatique -
Inria, a public scientific and technological establishment, having its
principal place of business at Domaine de Voluceau, Rocquencourt, BP
105, 78153 Le Chesnay cedex, France.


    Preamble

The purpose of this Free Software license agreement is to grant users
the right to modify and redistribute the software governed by this
license within the framework of an open source distribution model.

The exercising of this right is conditional upon certain obligations for
users so as to preserve this status for all subsequent redistributions.

In consideration of access to the source code and the rights to copy,
modify and redistribute granted by the license, users are provided only
with a limited warranty and the software&#x27;s author, the holder of the
economic rights, and the successive licensors only have limited liability.

In this respect, the risks associated with loading, using, modifying
and/or developing or reproducing the software by the user are brought to
the user&#x27;s attention, given its Free Software status, which may make it
complicated to use, with the result that its use is reserved for
developers and experienced professionals having in-depth computer
knowledge. Users are therefore encouraged to load and test the
suitability of the software as regards their requirements in conditions
enabling the security of their systems and/or data to be ensured and,
more generally, to use and operate it in the same conditions of
security. This Agreement may be freely reproduced and published,
provided it is not altered, and that no provisions are either added or
removed herefrom.

This Agreement may apply to any or all software for which the holder of
the economic rights decides to submit the use thereof to its provisions.

Frequently asked questions can be found on the official website of the
CeCILL licenses family (http://www.cecill.info/index.en.html) for any
necessary clarification.


    Article 1 - DEFINITIONS

For the purpose of this Agreement, when the following expressions
commence with a capital letter, they shall have the following meaning:

Agreement: means this license agreement, and its possible subsequent
versions and annexes.

Software: means the software in its Object Code and/or Source Code form
and, where applicable, its documentation, &quot;as is&quot; when the Licensee
accepts the Agreement.

Initial Software: means the Software in its Source Code and possibly its
Object Code form and, where applicable, its documentation, &quot;as is&quot; when
it is first distributed under the terms and conditions of the Agreement.

Modified Software: means the Software modified by at least one
Contribution.

Source Code: means all the Software&#x27;s instructions and program lines to
which access is required so as to modify the Software.

Object Code: means the binary files originating from the compilation of
the Source Code.

Holder: means the holder(s) of the economic rights over the Initial
Software.

Licensee: means the Software user(s) having accepted the Agreement.

Contributor: means a Licensee having made at least one Contribution.

Licensor: means the Holder, or any other individual or legal entity, who
distributes the Software under the Agreement.

Contribution: means any or all modifications, corrections, translations,
adaptations and/or new functions integrated into the Software by any or
all Contributors, as well as any or all Internal Modules.

Module: means a set of sources files including their documentation that
enables supplementary functions or services in addition to those offered
by the Software.

External Module: means any or all Modules, not derived from the
Software, so that this Module and the Software run in separate address
spaces, with one calling the other when they are run.

Internal Module: means any or all Module, connected to the Software so
that they both execute in the same address space.

GNU GPL: means the GNU General Public License version 2 or any
subsequent version, as published by the Free Software Foundation Inc.

GNU Affero GPL: means the GNU Affero General Public License version 3 or
any subsequent version, as published by the Free Software Foundation Inc.

EUPL: means the European Union Public License version 1.1 or any
subsequent version, as published by the European Commission.

Parties: mean both the Licensee and the Licensor.

These expressions may be used both in singular and plural form.


    Article 2 - PURPOSE

The purpose of the Agreement is the grant by the Licensor to the
Licensee of a non-exclusive, transferable and worldwide license for the
Software as set forth in Article 5 &lt;#scope&gt; hereinafter for the whole
term of the protection granted by the rights over said Software.


    Article 3 - ACCEPTANCE

3.1 The Licensee shall be deemed as having accepted the terms and
conditions of this Agreement upon the occurrence of the first of the
following events:

  * (i) loading the Software by any or all means, notably, by
    downloading from a remote server, or by loading from a physical medium;
  * (ii) the first time the Licensee exercises any of the rights granted
    hereunder.

3.2 One copy of the Agreement, containing a notice relating to the
characteristics of the Software, to the limited warranty, and to the
fact that its use is restricted to experienced users has been provided
to the Licensee prior to its acceptance as set forth in Article 3.1
&lt;#accepting&gt; hereinabove, and the Licensee hereby acknowledges that it
has read and understood it.


    Article 4 - EFFECTIVE DATE AND TERM


      4.1 EFFECTIVE DATE

The Agreement shall become effective on the date when it is accepted by
the Licensee as set forth in Article 3.1 &lt;#accepting&gt;.


      4.2 TERM

The Agreement shall remain in force for the entire legal term of
protection of the economic rights over the Software.


    Article 5 - SCOPE OF RIGHTS GRANTED

The Licensor hereby grants to the Licensee, who accepts, the following
rights over the Software for any or all use, and for the term of the
Agreement, on the basis of the terms and conditions set forth hereinafter.

Besides, if the Licensor owns or comes to own one or more patents
protecting all or part of the functions of the Software or of its
components, the Licensor undertakes not to enforce the rights granted by
these patents against successive Licensees using, exploiting or
modifying the Software. If these patents are transferred, the Licensor
undertakes to have the transferees subscribe to the obligations set
forth in this paragraph.


      5.1 RIGHT OF USE

The Licensee is authorized to use the Software, without any limitation
as to its fields of application, with it being hereinafter specified
that this comprises:

 1. permanent or temporary reproduction of all or part of the Software
    by any or all means and in any or all form.

 2. loading, displaying, running, or storing the Software on any or all
    medium.

 3. entitlement to observe, study or test its operation so as to
    determine the ideas and principles behind any or all constituent
    elements of said Software. This shall apply when the Licensee
    carries out any or all loading, displaying, running, transmission or
    storage operation as regards the Software, that it is entitled to
    carry out hereunder.


      5.2 ENTITLEMENT TO MAKE CONTRIBUTIONS

The right to make Contributions includes the right to translate, adapt,
arrange, or make any or all modifications to the Software, and the right
to reproduce the resulting software.

The Licensee is authorized to make any or all Contributions to the
Software provided that it includes an explicit notice that it is the
author of said Contribution and indicates the date of the creation thereof.


      5.3 RIGHT OF DISTRIBUTION

In particular, the right of distribution includes the right to publish,
transmit and communicate the Software to the general public on any or
all medium, and by any or all means, and the right to market, either in
consideration of a fee, or free of charge, one or more copies of the
Software by any means.

The Licensee is further authorized to distribute copies of the modified
or unmodified Software to third parties according to the terms and
conditions set forth hereinafter.


        5.3.1 DISTRIBUTION OF SOFTWARE WITHOUT MODIFICATION

The Licensee is authorized to distribute true copies of the Software in
Source Code or Object Code form, provided that said distribution
complies with all the provisions of the Agreement and is accompanied by:

 1. a copy of the Agreement,

 2. a notice relating to the limitation of both the Licensor&#x27;s warranty
    and liability as set forth in Articles 8 and 9,

and that, in the event that only the Object Code of the Software is
redistributed, the Licensee allows effective access to the full Source
Code of the Software for a period of at least three years from the
distribution of the Software, it being understood that the additional
acquisition cost of the Source Code shall not exceed the cost of the
data transfer.


        5.3.2 DISTRIBUTION OF MODIFIED SOFTWARE

When the Licensee makes a Contribution to the Software, the terms and
conditions for the distribution of the resulting Modified Software
become subject to all the provisions of this Agreement.

The Licensee is authorized to distribute the Modified Software, in
source code or object code form, provided that said distribution
complies with all the provisions of the Agreement and is accompanied by:

 1. a copy of the Agreement,

 2. a notice relating to the limitation of both the Licensor&#x27;s warranty
    and liability as set forth in Articles 8 and 9,

and, in the event that only the object code of the Modified Software is
redistributed,

 3. a note stating the conditions of effective access to the full source
    code of the Modified Software for a period of at least three years
    from the distribution of the Modified Software, it being understood
    that the additional acquisition cost of the source code shall not
    exceed the cost of the data transfer.


        5.3.3 DISTRIBUTION OF EXTERNAL MODULES

When the Licensee has developed an External Module, the terms and
conditions of this Agreement do not apply to said External Module, that
may be distributed under a separate license agreement.


        5.3.4 COMPATIBILITY WITH OTHER LICENSES

The Licensee can include a code that is subject to the provisions of one
of the versions of the GNU GPL, GNU Affero GPL and/or EUPL in the
Modified or unmodified Software, and distribute that entire code under
the terms of the same version of the GNU GPL, GNU Affero GPL and/or EUPL.

The Licensee can include the Modified or unmodified Software in a code
that is subject to the provisions of one of the versions of the GNU GPL,
GNU Affero GPL and/or EUPL and distribute that entire code under the
terms of the same version of the GNU GPL, GNU Affero GPL and/or EUPL.


    Article 6 - INTELLECTUAL PROPERTY


      6.1 OVER THE INITIAL SOFTWARE

The Holder owns the economic rights over the Initial Software. Any or
all use of the Initial Software is subject to compliance with the terms
and conditions under which the Holder has elected to distribute its work
and no one shall be entitled to modify the terms and conditions for the
distribution of said Initial Software.

The Holder undertakes that the Initial Software will remain ruled at
least by this Agreement, for the duration set forth in Article 4.2 &lt;#term&gt;.


      6.2 OVER THE CONTRIBUTIONS

The Licensee who develops a Contribution is the owner of the
intellectual property rights over this Contribution as defined by
applicable law.


      6.3 OVER THE EXTERNAL MODULES

The Licensee who develops an External Module is the owner of the
intellectual property rights over this External Module as defined by
applicable law and is free to choose the type of agreement that shall
govern its distribution.


      6.4 JOINT PROVISIONS

The Licensee expressly undertakes:

 1. not to remove, or modify, in any manner, the intellectual property
    notices attached to the Software;

 2. to reproduce said notices, in an identical manner, in the copies of
    the Software modified or not.

The Licensee undertakes not to directly or indirectly infringe the
intellectual property rights on the Software of the Holder and/or
Contributors, and to take, where applicable, vis-à-vis its staff, any
and all measures required to ensure respect of said intellectual
property rights of the Holder and/or Contributors.


    Article 7 - RELATED SERVICES

7.1 Under no circumstances shall the Agreement oblige the Licensor to
provide technical assistance or maintenance services for the Software.

However, the Licensor is entitled to offer this type of services. The
terms and conditions of such technical assistance, and/or such
maintenance, shall be set forth in a separate instrument. Only the
Licensor offering said maintenance and/or technical assistance services
shall incur liability therefor.

7.2 Similarly, any Licensor is entitled to offer to its licensees, under
its sole responsibility, a warranty, that shall only be binding upon
itself, for the redistribution of the Software and/or the Modified
Software, under terms and conditions that it is free to decide. Said
warranty, and the financial terms and conditions of its application,
shall be subject of a separate instrument executed between the Licensor
and the Licensee.


    Article 8 - LIABILITY

8.1 Subject to the provisions of Article 8.2, the Licensee shall be
entitled to claim compensation for any direct loss it may have suffered
from the Software as a result of a fault on the part of the relevant
Licensor, subject to providing evidence thereof.

8.2 The Licensor&#x27;s liability is limited to the commitments made under
this Agreement and shall not be incurred as a result of in particular:
(i) loss due the Licensee&#x27;s total or partial failure to fulfill its
obligations, (ii) direct or consequential loss that is suffered by the
Licensee due to the use or performance of the Software, and (iii) more
generally, any consequential loss. In particular the Parties expressly
agree that any or all pecuniary or business loss (i.e. loss of data,
loss of profits, operating loss, loss of customers or orders,
opportunity cost, any disturbance to business activities) or any or all
legal proceedings instituted against the Licensee by a third party,
shall constitute consequential loss and shall not provide entitlement to
any or all compensation from the Licensor.


    Article 9 - WARRANTY

9.1 The Licensee acknowledges that the scientific and technical
state-of-the-art when the Software was distributed did not enable all
possible uses to be tested and verified, nor for the presence of
possible defects to be detected. In this respect, the Licensee&#x27;s
attention has been drawn to the risks associated with loading, using,
modifying and/or developing and reproducing the Software which are
reserved for experienced users.

The Licensee shall be responsible for verifying, by any or all means,
the suitability of the product for its requirements, its good working
order, and for ensuring that it shall not cause damage to either persons
or properties.

9.2 The Licensor hereby represents, in good faith, that it is entitled
to grant all the rights over the Software (including in particular the
rights set forth in Article 5 &lt;#scope&gt;).

9.3 The Licensee acknowledges that the Software is supplied &quot;as is&quot; by
the Licensor without any other express or tacit warranty, other than
that provided for in Article 9.2 &lt;#good-faith&gt; and, in particular,
without any warranty as to its commercial value, its secured, safe,
innovative or relevant nature.

Specifically, the Licensor does not warrant that the Software is free
from any error, that it will operate without interruption, that it will
be compatible with the Licensee&#x27;s own equipment and software
configuration, nor that it will meet the Licensee&#x27;s requirements.

9.4 The Licensor does not either expressly or tacitly warrant that the
Software does not infringe any third party intellectual property right
relating to a patent, software or any other property right. Therefore,
the Licensor disclaims any and all liability towards the Licensee
arising out of any or all proceedings for infringement that may be
instituted in respect of the use, modification and redistribution of the
Software. Nevertheless, should such proceedings be instituted against
the Licensee, the Licensor shall provide it with technical and legal
expertise for its defense. Such technical and legal expertise shall be
decided on a case-by-case basis between the relevant Licensor and the
Licensee pursuant to a memorandum of understanding. The Licensor
disclaims any and all liability as regards the Licensee&#x27;s use of the
name of the Software. No warranty is given as regards the existence of
prior rights over the name of the Software or as regards the existence
of a trademark.


    Article 10 - TERMINATION

10.1 In the event of a breach by the Licensee of its obligations
hereunder, the Licensor may automatically terminate this Agreement
thirty (30) days after notice has been sent to the Licensee and has
remained ineffective.

10.2 A Licensee whose Agreement is terminated shall no longer be
authorized to use, modify or distribute the Software. However, any
licenses that it may have granted prior to termination of the Agreement
shall remain valid subject to their having been granted in compliance
with the terms and conditions hereof.


    Article 11 - MISCELLANEOUS


      11.1 EXCUSABLE EVENTS

Neither Party shall be liable for any or all delay, or failure to
perform the Agreement, that may be attributable to an event of force
majeure, an act of God or an outside cause, such as defective
functioning or interruptions of the electricity or telecommunications
networks, network paralysis following a virus attack, intervention by
government authorities, natural disasters, water damage, earthquakes,
fire, explosions, strikes and labor unrest, war, etc.

11.2 Any failure by either Party, on one or more occasions, to invoke
one or more of the provisions hereof, shall under no circumstances be
interpreted as being a waiver by the interested Party of its right to
invoke said provision(s) subsequently.

11.3 The Agreement cancels and replaces any or all previous agreements,
whether written or oral, between the Parties and having the same
purpose, and constitutes the entirety of the agreement between said
Parties concerning said purpose. No supplement or modification to the
terms and conditions hereof shall be effective as between the Parties
unless it is made in writing and signed by their duly authorized
representatives.

11.4 In the event that one or more of the provisions hereof were to
conflict with a current or future applicable act or legislative text,
said act or legislative text shall prevail, and the Parties shall make
the necessary amendments so as to comply with said act or legislative
text. All other provisions shall remain effective. Similarly, invalidity
of a provision of the Agreement, for any reason whatsoever, shall not
cause the Agreement as a whole to be invalid.


      11.5 LANGUAGE

The Agreement is drafted in both French and English and both versions
are deemed authentic.


    Article 12 - NEW VERSIONS OF THE AGREEMENT

12.1 Any person is authorized to duplicate and distribute copies of this
Agreement.

12.2 So as to ensure coherence, the wording of this Agreement is
protected and may only be modified by the authors of the License, who
reserve the right to periodically publish updates or new versions of the
Agreement, each with a separate number. These subsequent versions may
address new issues encountered by Free Software.

12.3 Any Software distributed under a given version of the Agreement may
only be subsequently distributed under the same version of the Agreement
or a subsequent version, subject to the provisions of Article 5.3.4
&lt;#compatibility&gt;.


    Article 13 - GOVERNING LAW AND JURISDICTION

13.1 The Agreement is governed by French law. The Parties agree to
endeavor to seek an amicable solution to any disagreements or disputes
that may arise during the performance of the Agreement.

13.2 Failing an amicable solution within two (2) months as from their
occurrence, and unless emergency proceedings are necessary, the
disagreements or disputes shall be referred to the Paris Courts having
jurisdiction, by the more diligent Party.
</pre>
            </li>
            <li class="license">
                <h3 id="ISC">ISC License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/nagisa/rust_libloading/ ">libloading 0.7.4</a></li>
                    <li><a href=" https://github.com/nagisa/rust_libloading/ ">libloading 0.8.8</a></li>
                </ul>
                <pre class="license-text">Copyright © 2015, Simonas Kazlauskas

Permission to use, copy, modify, and/or distribute this software for any purpose with or without
fee is hereby granted, provided that the above copyright notice and this permission notice appear
in all copies.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot; AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/haraldh/rust_uds_windows ">uds_windows 1.1.0</a></li>
                </ul>
                <pre class="license-text">    MIT License

    Copyright (c) Microsoft Corporation. All rights reserved.

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the &quot;Software&quot;), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RustCrypto/hashes ">sha1 0.10.6</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2006-2009 Graydon Hoare
Copyright (c) 2009-2013 Mozilla Foundation
Copyright (c) 2016 Artyom Pavlov

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-lang-nursery/lazy-static.rs ">lazy_static 1.5.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2010 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/servo/cgl-rs ">cgl 0.3.2</a></li>
                    <li><a href=" https://github.com/servo/core-foundation-rs ">cocoa-foundation 0.1.2</a></li>
                    <li><a href=" https://github.com/servo/core-foundation-rs ">cocoa 0.24.1</a></li>
                    <li><a href=" https://github.com/servo/core-foundation-rs ">core-foundation-sys 0.8.7</a></li>
                    <li><a href=" https://github.com/servo/core-foundation-rs ">core-foundation 0.9.4</a></li>
                    <li><a href=" https://github.com/servo/core-foundation-rs ">core-graphics-types 0.1.3</a></li>
                    <li><a href=" https://github.com/servo/core-foundation-rs ">core-graphics 0.22.3</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2012-2013 Mozilla Foundation

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/KokaKiwi/rust-hex ">hex 0.4.3</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2013-2014 The Rust Project Developers.
Copyright (c) 2015-2020 The rust-hex Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/servo/rust-url ">form_urlencoded 1.2.2</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2013-2016 The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/servo/rust-url/ ">idna 1.1.0</a></li>
                    <li><a href=" https://github.com/servo/rust-url/ ">percent-encoding 2.3.2</a></li>
                    <li><a href=" https://github.com/servo/rust-url ">url 2.5.7</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2013-2025 The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-lang/cc-rs ">cc 1.2.36</a></li>
                    <li><a href=" https://github.com/rust-lang/cfg-if ">cfg-if 1.0.3</a></li>
                    <li><a href=" https://github.com/rust-lang/cc-rs ">find-msvc-tools 0.1.1</a></li>
                    <li><a href=" https://github.com/rust-lang/jobserver-rs ">jobserver 0.1.34</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/js-sys ">js-sys 0.3.78</a></li>
                    <li><a href=" https://github.com/rust-lang/pkg-config-rs ">pkg-config 0.3.32</a></li>
                    <li><a href=" https://github.com/alexcrichton/scoped-tls ">scoped-tls 1.0.1</a></li>
                    <li><a href=" https://github.com/rust-lang/socket2 ">socket2 0.4.10</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/backend ">wasm-bindgen-backend 0.2.101</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/futures ">wasm-bindgen-futures 0.4.51</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro-support ">wasm-bindgen-macro-support 0.2.101</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro ">wasm-bindgen-macro 0.2.101</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared ">wasm-bindgen-shared 0.2.101</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen ">wasm-bindgen 0.2.101</a></li>
                    <li><a href=" https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/web-sys ">web-sys 0.3.78</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/tokio-rs/mio ">mio 0.8.11</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2014 Carl Lerche and other MIO contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/lambda-fairy/rust-errno ">errno 0.3.14</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2014 Chris Wong

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/bitflags/bitflags ">bitflags 1.3.2</a></li>
                    <li><a href=" https://github.com/bitflags/bitflags ">bitflags 2.9.4</a></li>
                    <li><a href=" https://github.com/rust-lang/log ">log 0.4.28</a></li>
                    <li><a href=" https://github.com/rust-num/num-traits ">num-traits 0.2.19</a></li>
                    <li><a href=" https://github.com/rust-lang/regex ">regex-automata 0.4.10</a></li>
                    <li><a href=" https://github.com/rust-lang/regex ">regex-syntax 0.8.6</a></li>
                    <li><a href=" https://github.com/rust-lang/regex ">regex 1.11.2</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-lang/libc ">libc 0.2.175</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2014-2020 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-lang/flate2-rs ">flate2 1.1.2</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2014-2025 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/Stebalien/tempfile ">tempfile 3.22.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2015 Steven Allen

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/contain-rs/vec-map ">vec_map 0.8.2</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2015 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/sfackler/rust-jni-sys ">jni-sys 0.3.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2015 The rust-jni-sys Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/elinorbgr/dlib ">dlib 0.5.2</a></li>
                    <li><a href=" https://github.com/smithay/wayland-rs ">wayland-client 0.29.5</a></li>
                    <li><a href=" https://github.com/smithay/wayland-rs ">wayland-commons 0.29.5</a></li>
                    <li><a href=" https://github.com/smithay/wayland-rs ">wayland-cursor 0.29.5</a></li>
                    <li><a href=" https://github.com/smithay/wayland-rs ">wayland-protocols 0.29.5</a></li>
                    <li><a href=" https://github.com/smithay/wayland-rs ">wayland-scanner 0.29.5</a></li>
                    <li><a href=" https://github.com/smithay/wayland-rs ">wayland-sys 0.29.5</a></li>
                    <li><a href=" https://github.com/smithay/wayland-rs ">wayland-sys 0.30.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2015 Victor Berger

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/image-rs/image-png ">png 0.17.16</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2015 nwin

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/retep998/winapi-rs ">winapi 0.3.9</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2015-2018 The winapi-rs Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-lang/futures-rs ">futures-core 0.3.31</a></li>
                    <li><a href=" https://github.com/rust-lang/futures-rs ">futures-io 0.3.31</a></li>
                    <li><a href=" https://github.com/rust-lang/futures-rs ">futures-sink 0.3.31</a></li>
                    <li><a href=" https://github.com/rust-lang/futures-rs ">futures-task 0.3.31</a></li>
                    <li><a href=" https://github.com/rust-lang/futures-rs ">futures-util 0.3.31</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-lang/hashbrown ">hashbrown 0.15.5</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016 Amanieu d&#x27;Antras

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/alacritty/vte ">utf8parse 0.2.2</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016 Joe Wilm

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/mcarton/rust-derivative ">derivative 2.2.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016 Martin Carton

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the &quot;Software&quot;), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/Amanieu/parking_lot ">lock_api 0.4.13</a></li>
                    <li><a href=" https://github.com/Amanieu/parking_lot ">parking_lot 0.12.4</a></li>
                    <li><a href=" https://github.com/Amanieu/parking_lot ">parking_lot_core 0.9.11</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/indexmap-rs/indexmap ">indexmap 2.11.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016--2017

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/indexmap-rs/equivalent ">equivalent 1.0.2</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016--2023

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/bluss/scopeguard ">scopeguard 1.2.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2016-2019 Ulrik Sverdrup &quot;bluss&quot; and scopeguard developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RustCrypto/traits ">digest 0.10.7</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017 Artyom Pavlov

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/Gilnaa/memoffset ">memoffset 0.6.5</a></li>
                    <li><a href=" https://github.com/Gilnaa/memoffset ">memoffset 0.7.1</a></li>
                    <li><a href=" https://github.com/Gilnaa/memoffset ">memoffset 0.9.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017 Gilad Naaman

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/meithecatte/enumflags2 ">enumflags2_derive 0.7.12</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017 Maik Klein

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://gitlab.redox-os.org/redox-os/syscall ">redox_syscall 0.3.5</a></li>
                    <li><a href=" https://gitlab.redox-os.org/redox-os/syscall ">redox_syscall 0.5.17</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017 Redox OS Developers

MIT License

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
&quot;Software&quot;), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/storyyeller/stable_deref_trait ">stable_deref_trait 1.2.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017 Robert Grosse

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/sfackler/foreign-types ">foreign-types-shared 0.1.1</a></li>
                    <li><a href=" https://github.com/sfackler/foreign-types ">foreign-types 0.3.2</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017 The foreign-types Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/vorner/signal-hook ">signal-hook-registry 1.4.6</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017 tokio-jsonrpc developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/meithecatte/enumflags2 ">enumflags2 0.7.12</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2017-2023 Maik Klein, Maja Kądziołka

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/tokio-rs/bytes ">bytes 1.10.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 Carl Lerche

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/cuviper/autocfg ">autocfg 1.5.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 Josh Stone

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/smithay/smithay-clipboard ">smithay-clipboard 0.6.6</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 Lucas Timmins &amp; Victor Berger

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/servo/rust-smallvec ">smallvec 1.15.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 The Servo Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-lang-nursery/pin-utils ">pin-utils 0.1.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 The pin-utils authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/tkaitchuck/ahash ">ahash 0.8.12</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 Tom Kaitchuck

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/smithay/client-toolkit ">smithay-client-toolkit 0.16.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 Victor Berger

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/Smithay/calloop ">calloop 0.10.6</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 Victor Berger

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/harfbuzz/ttf-parser ">ttf-parser 0.25.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018 Yevhenii Reizner

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RustCrypto/utils ">block-buffer 0.10.4</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018-2019 The RustCrypto Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-random/getrandom ">getrandom 0.2.16</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018-2024 The rust-random Project Developers
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-random/getrandom ">getrandom 0.3.3</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2018-2025 The rust-random Project Developers
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/tokio-rs/slab ">slab 0.4.11</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2019 Carl Lerche

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/fitzgen/bumpalo ">bumpalo 3.19.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2019 Nick Fitzgerald

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/cryptocorrosion/cryptocorrosion ">ppv-lite86 0.2.21</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2019 The CryptoCorrosion Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/tokio-rs/tracing ">tracing-attributes 0.1.30</a></li>
                    <li><a href=" https://github.com/tokio-rs/tracing ">tracing-core 0.1.34</a></li>
                    <li><a href=" https://github.com/tokio-rs/tracing ">tracing 0.1.41</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2019 Tokio Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/marcianx/downcast-rs ">downcast-rs 1.2.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2020 Ashish Myles and contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RazrFalcon/memmap2-rs ">memmap2 0.5.10</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2020 Yevhenii Reizner
Copyright (c) 2015 Dan Burkert

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RustCrypto/utils ">cpufeatures 0.2.17</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2020-2025 The RustCrypto Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RustCrypto/traits ">crypto-common 0.1.6</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2021 RustCrypto Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/odilia-app/atspi ">atspi-common 0.3.0</a></li>
                    <li><a href=" https://github.com/odilia-app/atspi/ ">atspi-connection 0.3.0</a></li>
                    <li><a href=" https://github.com/odilia-app/atspi ">atspi 0.19.0</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2022 Tait Hoyem &lt;tait@tait.tech&gt;

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/RazrFalcon/strict-num ">strict-num 0.1.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2022 Yevhenii Reizner

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/danieldg/async-once-cell ">async-once-cell 0.5.4</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2023 Daniel De Graaf

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-cli/anstyle.git ">anstream 0.6.20</a></li>
                    <li><a href=" https://github.com/rust-cli/anstyle.git ">anstyle-parse 0.2.7</a></li>
                    <li><a href=" https://github.com/rust-cli/anstyle.git ">anstyle-query 1.1.4</a></li>
                    <li><a href=" https://github.com/rust-cli/anstyle.git ">anstyle-wincon 3.0.10</a></li>
                    <li><a href=" https://github.com/rust-cli/anstyle.git ">anstyle 1.0.11</a></li>
                    <li><a href=" https://github.com/rust-cli/anstyle.git ">colorchoice 1.0.4</a></li>
                    <li><a href=" https://github.com/rust-cli/env_logger ">env_filter 0.1.3</a></li>
                    <li><a href=" https://github.com/rust-cli/env_logger ">env_logger 0.11.8</a></li>
                    <li><a href=" https://github.com/polyfill-rs/is_terminal_polyfill ">is_terminal_polyfill 1.70.1</a></li>
                    <li><a href=" https://github.com/polyfill-rs/once_cell_polyfill ">once_cell_polyfill 1.70.1</a></li>
                    <li><a href=" https://github.com/toml-rs/toml ">toml_datetime 0.6.11</a></li>
                    <li><a href=" https://github.com/toml-rs/toml ">toml_edit 0.19.15</a></li>
                </ul>
                <pre class="license-text">Copyright (c) Individual contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/hsivonen/idna_adapter ">idna_adapter 1.2.1</a></li>
                </ul>
                <pre class="license-text">Copyright (c) The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/bluss/arrayvec ">arrayvec 0.7.6</a></li>
                </ul>
                <pre class="license-text">Copyright (c) Ulrik Sverdrup &quot;bluss&quot; 2015-2023

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/mystor/synstructure ">synstructure 0.13.2</a></li>
                </ul>
                <pre class="license-text">Copyright 2016 Nika Layzell

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the &quot;Software&quot;), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-random/rand ">rand 0.8.5</a></li>
                    <li><a href=" https://github.com/rust-random/rand ">rand_chacha 0.3.1</a></li>
                    <li><a href=" https://github.com/rust-random/rand ">rand_core 0.6.4</a></li>
                </ul>
                <pre class="license-text">Copyright 2018 Developers of the Rand project
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/paritytech/nohash-hasher ">nohash-hasher 0.2.0</a></li>
                </ul>
                <pre class="license-text">Copyright 2018 Parity Technologies (UK) Ltd.

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the &quot;Software&quot;), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS
OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/psychon/x11rb ">x11rb-protocol 0.13.2</a></li>
                    <li><a href=" https://github.com/psychon/x11rb ">x11rb 0.13.2</a></li>
                </ul>
                <pre class="license-text">Copyright 2019 x11rb Contributers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/google/zerocopy ">zerocopy 0.8.27</a></li>
                </ul>
                <pre class="license-text">Copyright 2023 The Fuchsia Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/hsivonen/utf8_iter ">utf8_iter 1.0.4</a></li>
                </ul>
                <pre class="license-text">Copyright Mozilla Foundation

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-windowing/glutin ">glutin-winit 0.3.0</a></li>
                </ul>
                <pre class="license-text">Copyright © 2022 Kirill Chibisov

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the “Software”), to deal
in the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/emilk/egui/tree/master/crates/epaint ">epaint 0.24.1</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2014 John Slegers

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the &quot;Software&quot;), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/nvzqz/static-assertions-rs ">static_assertions 1.1.0</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2017 Nikolai Vazquez

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/srijs/rust-crc32fast ">crc32fast 1.5.0</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2018 Sam Rijs, Alex Crichton and contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/Lokathor/bytemuck ">bytemuck 1.23.2</a></li>
                    <li><a href=" https://github.com/Lokathor/bytemuck ">bytemuck_derive 1.10.1</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2019 Daniel &quot;Lokathor&quot; Gee.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the &quot;Software&quot;), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-windowing/raw-window-handle ">raw-window-handle 0.5.2</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2019 Osspial

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/katharostech/cfg_aliases ">cfg_aliases 0.1.1</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2020 Katharos Technology

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the &quot;Software&quot;), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/miklelappo/android-properties ">android-properties 0.2.2</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2020 Mikhail Lappo

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/esposm03/xcursor-rs ">xcursor 0.3.10</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2020 Samuele Esposito

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/PolyMeilex/sctk-adwaita ">sctk-adwaita 0.5.4</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2022 Bartłomiej Maryńczak

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/1Password/arboard ">arboard 3.6.1</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2022 The Arboard contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://gitlab.redox-os.org/redox-os/libredox.git ">libredox 0.1.9</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2023 4lDO2

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/daxpedda/web-time ">web-time 0.2.4</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) 2023 dAxpeDDa

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/AccessKit/accesskit ">accesskit 0.12.3</a></li>
                    <li><a href=" https://github.com/AccessKit/accesskit ">accesskit_consumer 0.16.1</a></li>
                    <li><a href=" https://github.com/AccessKit/accesskit ">accesskit_macos 0.10.1</a></li>
                    <li><a href=" https://github.com/AccessKit/accesskit ">accesskit_unix 0.6.2</a></li>
                    <li><a href=" https://github.com/AccessKit/accesskit ">accesskit_windows 0.15.1</a></li>
                    <li><a href=" https://github.com/odilia-app/atspi ">atspi-proxies 0.3.0</a></li>
                    <li><a href=" http://github.com/SSheldon/rust-block ">block 0.1.6</a></li>
                    <li><a href=" https://github.com/emk/cesu8-rs ">cesu8 1.1.0</a></li>
                    <li><a href=" http://github.com/SSheldon/rust-dispatch ">dispatch 0.2.0</a></li>
                    <li><a href=" https://github.com/emilk/egui ">ecolor 0.24.1</a></li>
                    <li><a href=" https://github.com/emilk/egui/tree/master/crates/eframe ">eframe 0.24.1</a></li>
                    <li><a href=" https://github.com/emilk/egui/tree/master/crates/egui-winit ">egui-winit 0.24.1</a></li>
                    <li><a href=" https://github.com/emilk/egui ">egui 0.24.1</a></li>
                    <li><a href=" https://github.com/emilk/egui/tree/master/crates/egui_glow ">egui_glow 0.24.1</a></li>
                    <li><a href=" https://github.com/emilk/egui/tree/master/crates/emath ">emath 0.24.1</a></li>
                    <li><a href=" https://github.com/SSheldon/malloc_buf ">malloc_buf 0.0.6</a></li>
                    <li><a href=" https://github.com/rust-windowing/android-ndk-rs ">ndk-context 0.1.1</a></li>
                    <li><a href=" https://github.com/rust-windowing/android-ndk-rs ">ndk-sys 0.4.1+23.1.7779620</a></li>
                    <li><a href=" https://github.com/rust-windowing/android-ndk-rs ">ndk 0.7.0</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc-sys 0.2.0-beta.2</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2-app-kit 0.3.1</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2-core-foundation 0.3.1</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2-core-graphics 0.3.1</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2-encode 2.0.0-pre.2</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2-encode 4.1.0</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2-foundation 0.3.1</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2 0.3.0-beta.3.patch-leaks.3</a></li>
                    <li><a href=" https://github.com/madsmtm/objc2 ">objc2 0.6.2</a></li>
                    <li><a href=" https://github.com/r-efi/r-efi ">r-efi 5.3.0</a></li>
                    <li><a href=" https://github.com/bytecodealliance/wasi-rs ">wasip2 1.0.0+wasi-0.2.4</a></li>
                    <li><a href=" https://github.com/retep998/winapi-rs ">winapi-i686-pc-windows-gnu 0.4.0</a></li>
                    <li><a href=" https://github.com/retep998/winapi-rs ">winapi-x86_64-pc-windows-gnu 0.4.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-implement 0.48.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-interface 0.48.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-link 0.1.3</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-link 0.2.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-sys 0.45.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-sys 0.48.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-sys 0.59.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-sys 0.60.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-sys 0.61.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-targets 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-targets 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-targets 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows-targets 0.53.3</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows 0.48.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_gnullvm 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_gnullvm 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_gnullvm 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_gnullvm 0.53.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_msvc 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_msvc 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_msvc 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_aarch64_msvc 0.53.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_gnu 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_gnu 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_gnu 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_gnu 0.53.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_gnullvm 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_gnullvm 0.53.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_msvc 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_msvc 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_msvc 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_i686_msvc 0.53.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnu 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnu 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnu 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnu 0.53.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnullvm 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnullvm 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnullvm 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_gnullvm 0.53.0</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_msvc 0.42.2</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_msvc 0.48.5</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_msvc 0.52.6</a></li>
                    <li><a href=" https://github.com/microsoft/windows-rs ">windows_x86_64_msvc 0.53.0</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) &lt;year&gt; &lt;copyright holders&gt;

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the &quot;Software&quot;), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the
following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT
LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO
EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" http://github.com/SSheldon/rust-objc ">objc 0.2.7</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) Steven Sheldon

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/mcountryman/simd-adler32 ">simd-adler32 0.3.7</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright (c) [2021] [Marvin Countryman]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide ">miniz_oxide 0.8.9</a></li>
                </ul>
                <pre class="license-text">MIT License

Copyright 2013-2014 RAD Game Tools and Valve Software
Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC
Copyright (c) 2017 Frommi
Copyright (c) 2017-2024 oyvindln

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/image-rs/fdeflate ">fdeflate 0.3.7</a></li>
                    <li><a href=" https://github.com/image-rs/image ">image 0.24.9</a></li>
                </ul>
                <pre class="license-text">MIT License

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/rust-mobile/android-activity ">android-activity 0.4.3</a></li>
                </ul>
                <pre class="license-text">MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/dcchut/async-recursion ">async-recursion 1.1.1</a></li>
                </ul>
                <pre class="license-text">Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/oyvindln/adler2 ">adler2 2.0.1</a></li>
                    <li><a href=" https://github.com/smol-rs/async-channel ">async-channel 2.5.0</a></li>
                    <li><a href=" https://github.com/smol-rs/async-executor ">async-executor 1.13.3</a></li>
                    <li><a href=" https://github.com/smol-rs/async-fs ">async-fs 1.6.0</a></li>
                    <li><a href=" https://github.com/smol-rs/async-io ">async-io 1.13.0</a></li>
                    <li><a href=" https://github.com/smol-rs/async-io ">async-io 2.5.0</a></li>
                    <li><a href=" https://github.com/smol-rs/async-lock ">async-lock 2.8.0</a></li>
                    <li><a href=" https://github.com/smol-rs/async-lock ">async-lock 3.4.1</a></li>
                    <li><a href=" https://github.com/smol-rs/async-process ">async-process 1.8.1</a></li>
                    <li><a href=" https://github.com/smol-rs/async-signal ">async-signal 0.2.12</a></li>
                    <li><a href=" https://github.com/smol-rs/async-task ">async-task 4.7.1</a></li>
                    <li><a href=" https://github.com/dtolnay/async-trait ">async-trait 0.1.89</a></li>
                    <li><a href=" https://github.com/smol-rs/atomic-waker ">atomic-waker 1.1.2</a></li>
                    <li><a href=" https://github.com/smol-rs/blocking ">blocking 1.6.2</a></li>
                    <li><a href=" https://github.com/smol-rs/concurrent-queue ">concurrent-queue 2.5.0</a></li>
                    <li><a href=" https://github.com/yaahc/displaydoc ">displaydoc 0.2.5</a></li>
                    <li><a href=" https://github.com/smol-rs/event-listener-strategy ">event-listener-strategy 0.5.4</a></li>
                    <li><a href=" https://github.com/smol-rs/event-listener ">event-listener 2.5.3</a></li>
                    <li><a href=" https://github.com/smol-rs/event-listener ">event-listener 3.1.0</a></li>
                    <li><a href=" https://github.com/smol-rs/event-listener ">event-listener 5.4.1</a></li>
                    <li><a href=" https://github.com/smol-rs/fastrand ">fastrand 1.9.0</a></li>
                    <li><a href=" https://github.com/smol-rs/fastrand ">fastrand 2.3.0</a></li>
                    <li><a href=" https://github.com/smol-rs/futures-lite ">futures-lite 1.13.0</a></li>
                    <li><a href=" https://github.com/smol-rs/futures-lite ">futures-lite 2.6.1</a></li>
                    <li><a href=" https://github.com/hermit-os/hermit-rs ">hermit-abi 0.3.9</a></li>
                    <li><a href=" https://github.com/hermit-os/hermit-rs ">hermit-abi 0.5.2</a></li>
                    <li><a href=" https://github.com/rust-lang/cargo ">home 0.5.11</a></li>
                    <li><a href=" https://github.com/sunfishcode/io-lifetimes ">io-lifetimes 1.0.11</a></li>
                    <li><a href=" https://github.com/sunfishcode/linux-raw-sys ">linux-raw-sys 0.11.0</a></li>
                    <li><a href=" https://github.com/sunfishcode/linux-raw-sys ">linux-raw-sys 0.3.8</a></li>
                    <li><a href=" https://github.com/sunfishcode/linux-raw-sys ">linux-raw-sys 0.4.15</a></li>
                    <li><a href=" https://github.com/illicitonion/num_enum ">num_enum 0.5.11</a></li>
                    <li><a href=" https://github.com/illicitonion/num_enum ">num_enum 0.6.1</a></li>
                    <li><a href=" https://github.com/illicitonion/num_enum ">num_enum_derive 0.5.11</a></li>
                    <li><a href=" https://github.com/illicitonion/num_enum ">num_enum_derive 0.6.1</a></li>
                    <li><a href=" https://github.com/matklad/once_cell ">once_cell 1.21.3</a></li>
                    <li><a href=" https://github.com/danieldg/ordered-stream ">ordered-stream 0.2.0</a></li>
                    <li><a href=" https://github.com/smol-rs/parking ">parking 2.2.1</a></li>
                    <li><a href=" https://github.com/dtolnay/paste ">paste 1.0.15</a></li>
                    <li><a href=" https://github.com/taiki-e/pin-project-lite ">pin-project-lite 0.2.16</a></li>
                    <li><a href=" https://github.com/smol-rs/piper ">piper 0.2.4</a></li>
                    <li><a href=" https://github.com/smol-rs/polling ">polling 3.10.0</a></li>
                    <li><a href=" https://github.com/taiki-e/portable-atomic ">portable-atomic-util 0.2.4</a></li>
                    <li><a href=" https://github.com/taiki-e/portable-atomic ">portable-atomic 1.11.1</a></li>
                    <li><a href=" https://github.com/bkchr/proc-macro-crate ">proc-macro-crate 1.3.1</a></li>
                    <li><a href=" https://github.com/dtolnay/proc-macro2 ">proc-macro2 1.0.101</a></li>
                    <li><a href=" https://github.com/dtolnay/quote ">quote 1.0.40</a></li>
                    <li><a href=" https://github.com/bytecodealliance/rustix ">rustix 0.37.28</a></li>
                    <li><a href=" https://github.com/bytecodealliance/rustix ">rustix 0.38.44</a></li>
                    <li><a href=" https://github.com/bytecodealliance/rustix ">rustix 1.1.2</a></li>
                    <li><a href=" https://github.com/dtolnay/rustversion ">rustversion 1.0.22</a></li>
                    <li><a href=" https://github.com/serde-rs/serde ">serde 1.0.219</a></li>
                    <li><a href=" https://github.com/serde-rs/serde ">serde_derive 1.0.219</a></li>
                    <li><a href=" https://github.com/dtolnay/serde-repr ">serde_repr 0.1.20</a></li>
                    <li><a href=" https://github.com/dtolnay/syn ">syn 1.0.109</a></li>
                    <li><a href=" https://github.com/dtolnay/syn ">syn 2.0.106</a></li>
                    <li><a href=" https://github.com/dtolnay/thiserror ">thiserror-impl 1.0.69</a></li>
                    <li><a href=" https://github.com/dtolnay/thiserror ">thiserror 1.0.69</a></li>
                    <li><a href=" https://github.com/dtolnay/unicode-ident ">unicode-ident 1.0.19</a></li>
                    <li><a href=" https://github.com/smol-rs/waker-fn ">waker-fn 1.2.0</a></li>
                    <li><a href=" https://github.com/bytecodealliance/wasi ">wasi 0.11.1+wasi-snapshot-preview1</a></li>
                    <li><a href=" https://github.com/bytecodealliance/wasi-rs ">wasi 0.14.5+wasi-0.2.4</a></li>
                    <li><a href=" https://github.com/bytecodealliance/wit-bindgen ">wit-bindgen 0.45.1</a></li>
                    <li><a href=" https://github.com/AltF02/x11-rs.git ">x11-dl 2.21.0</a></li>
                    <li><a href=" https://github.com/zeenix/xdg-home ">xdg-home 1.3.0</a></li>
                    <li><a href=" https://github.com/dbus2/zbus/ ">zbus 3.15.2</a></li>
                    <li><a href=" https://github.com/dbus2/zbus/ ">zbus_macros 3.15.2</a></li>
                    <li><a href=" https://github.com/dbus2/zbus/ ">zbus_names 2.6.1</a></li>
                    <li><a href=" https://github.com/dbus2/zbus/ ">zvariant 3.15.2</a></li>
                    <li><a href=" https://github.com/dbus2/zbus/ ">zvariant_derive 3.15.2</a></li>
                    <li><a href=" https://github.com/dbus2/zbus/ ">zvariant_utils 1.0.1</a></li>
                </ul>
                <pre class="license-text">Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/grovesNL/glow ">glow 0.12.3</a></li>
                </ul>
                <pre class="license-text">Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/smol-rs/polling ">polling 2.8.0</a></li>
                </ul>
                <pre class="license-text">Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/winnow-rs/winnow ">winnow 0.5.40</a></li>
                </ul>
                <pre class="license-text">Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
&quot;Software&quot;), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/paholg/typenum ">typenum 1.18.0</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2014 Paho Lurie-Gregg

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/kornelski/xml-rs ">xml-rs 0.8.27</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2014 Vladimir Matveev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/BurntSushi/aho-corasick ">aho-corasick 1.1.3</a></li>
                    <li><a href=" https://github.com/BurntSushi/byteorder ">byteorder 1.5.0</a></li>
                    <li><a href=" https://github.com/BurntSushi/jiff ">jiff 0.2.15</a></li>
                    <li><a href=" https://github.com/BurntSushi/memchr ">memchr 2.7.5</a></li>
                    <li><a href=" https://github.com/BurntSushi/walkdir ">walkdir 2.5.0</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2015 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/nix-rust/nix ">nix 0.24.3</a></li>
                    <li><a href=" https://github.com/nix-rust/nix ">nix 0.25.1</a></li>
                    <li><a href=" https://github.com/nix-rust/nix ">nix 0.26.4</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2015 Carl Lerche + nix-rust Authors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/Marwes/combine ">combine 4.6.7</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2015 Markus Westerlind

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/comex/rust-shlex ">shlex 1.3.0</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2015 Nicholas Allegra (comex).

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://gitlab.redox-os.org/redox-os/orbclient ">orbclient 0.3.48</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2015-2019 Jeremy Soller

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/amodm/webbrowser-rs ">webbrowser 0.8.15</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2015-2022 Amod Malviya

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/image-rs/color_quant.git ">color_quant 1.1.0</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2016 PistonDevelopers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/jni-rs/jni-rs ">jni 0.21.1</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2016 Prevoty, Inc. and jni-rs contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/BurntSushi/same-file ">same-file 1.0.6</a></li>
                    <li><a href=" https://github.com/BurntSushi/winapi-util ">winapi-util 0.1.11</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2017 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/crossbeam-rs/crossbeam ">crossbeam-utils 0.8.21</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2019 The Crossbeam Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the &quot;Software&quot;), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/smol-rs/async-broadcast ">async-broadcast 0.5.1</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2020 Yoshua Wuyts

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/SergioBenitez/version_check ">version_check 0.9.5</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)
Copyright (c) 2017-2018 Sergio Benitez

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the &quot;Software&quot;), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="MIT">MIT License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/fizyk20/generic-array.git ">generic-array 0.14.7</a></li>
                </ul>
                <pre class="license-text">The MIT License (MIT)

Copyright (c) 2015 Bartłomiej Kamiński

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the &quot;Software&quot;), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.</pre>
            </li>
            <li class="license">
                <h3 id="OFL-1.1">SIL Open Font License 1.1</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/emilk/egui/tree/master/crates/epaint ">epaint 0.24.1</a></li>
                </ul>
                <pre class="license-text">This Font Software is licensed under the SIL Open Font License,
Version 1.1.

This license is copied below, and is also available with a FAQ at:
http://scripts.sil.org/OFL

-----------------------------------------------------------
SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007
-----------------------------------------------------------

PREAMBLE
The goals of the Open Font License (OFL) are to stimulate worldwide
development of collaborative font projects, to support the font
creation efforts of academic and linguistic communities, and to
provide a free and open framework in which fonts may be shared and
improved in partnership with others.

The OFL allows the licensed fonts to be used, studied, modified and
redistributed freely as long as they are not sold by themselves. The
fonts, including any derivative works, can be bundled, embedded,
redistributed and/or sold with any software provided that any reserved
names are not used by derivative works. The fonts and derivatives,
however, cannot be released under any other type of license. The
requirement for fonts to remain under this license does not apply to
any document created using the fonts or their derivatives.

DEFINITIONS
&quot;Font Software&quot; refers to the set of files released by the Copyright
Holder(s) under this license and clearly marked as such. This may
include source files, build scripts and documentation.

&quot;Reserved Font Name&quot; refers to any names specified as such after the
copyright statement(s).

&quot;Original Version&quot; refers to the collection of Font Software
components as distributed by the Copyright Holder(s).

&quot;Modified Version&quot; refers to any derivative made by adding to,
deleting, or substituting -- in part or in whole -- any of the
components of the Original Version, by changing formats or by porting
the Font Software to a new environment.

&quot;Author&quot; refers to any designer, engineer, programmer, technical
writer or other person who contributed to the Font Software.

PERMISSION &amp; CONDITIONS
Permission is hereby granted, free of charge, to any person obtaining
a copy of the Font Software, to use, study, copy, merge, embed,
modify, redistribute, and sell modified and unmodified copies of the
Font Software, subject to the following conditions:

1) Neither the Font Software nor any of its individual components, in
Original or Modified Versions, may be sold by itself.

2) Original or Modified Versions of the Font Software may be bundled,
redistributed and/or sold with any software, provided that each copy
contains the above copyright notice and this license. These can be
included either as stand-alone text files, human-readable headers or
in the appropriate machine-readable metadata fields within text or
binary files as long as those fields can be easily viewed by the user.

3) No Modified Version of the Font Software may use the Reserved Font
Name(s) unless explicit written permission is granted by the
corresponding Copyright Holder. This restriction only applies to the
primary font name as presented to the users.

4) The name(s) of the Copyright Holder(s) or the Author(s) of the Font
Software shall not be used to promote, endorse or advertise any
Modified Version, except to acknowledge the contribution(s) of the
Copyright Holder(s) and the Author(s) or with their explicit written
permission.

5) The Font Software, modified or unmodified, in part or in whole,
must be distributed entirely under this license, and must not be
distributed under any other license. The requirement for fonts to
remain under this license does not apply to any document created using
the Font Software.

TERMINATION
This license becomes null and void if any of the above conditions are
not met.

DISCLAIMER
THE FONT SOFTWARE IS PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE
COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
INCLUDING ANY GENERAL, SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL
DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM
OTHER DEALINGS IN THE FONT SOFTWARE.
</pre>
            </li>
            <li class="license">
                <h3 id="Unicode-3.0">Unicode License v3</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/dtolnay/unicode-ident ">unicode-ident 1.0.19</a></li>
                </ul>
                <pre class="license-text">UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2023 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the &quot;Data Files&quot;) or
software and any associated documentation (the &quot;Software&quot;) to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
</pre>
            </li>
            <li class="license">
                <h3 id="Unicode-3.0">Unicode License v3</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/unicode-org/icu4x ">icu_collections 2.0.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">icu_locale_core 2.0.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">icu_normalizer 2.0.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">icu_normalizer_data 2.0.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">icu_properties 2.0.1</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">icu_properties_data 2.0.1</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">icu_provider 2.0.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">litemap 0.8.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">potential_utf 0.1.3</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">tinystr 0.8.1</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">writeable 0.6.1</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">yoke-derive 0.8.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">yoke 0.8.0</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">zerofrom-derive 0.1.6</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">zerofrom 0.1.6</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">zerotrie 0.2.2</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">zerovec-derive 0.11.1</a></li>
                    <li><a href=" https://github.com/unicode-org/icu4x ">zerovec 0.11.4</a></li>
                </ul>
                <pre class="license-text">UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the &quot;Data Files&quot;) or
software and any associated documentation (the &quot;Software&quot;) to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED &quot;AS IS&quot;, WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.

SPDX-License-Identifier: Unicode-3.0

—

Portions of ICU4X may have been adapted from ICU4C and/or ICU4J.
ICU 1.8.1 to ICU 57.1 © 1995-2016 International Business Machines Corporation and others.
</pre>
            </li>
            <li class="license">
                <h3 id="Zlib">zlib License</h3>
                <h4>Used by:</h4>
                <ul class="license-used-by">
                    <li><a href=" https://github.com/orlp/slotmap ">slotmap 1.0.7</a></li>
                </ul>
                <pre class="license-text">Copyright (c) 2021 Orson Peters &lt;orsonpeters@gmail.com&gt;

This software is provided &#x27;as-is&#x27;, without any express or implied warranty. In
no event will the authors be held liable for any damages arising from the use of
this software.

Permission is granted to anyone to use this software for any purpose, including
commercial applications, and to alter it and redistribute it freely, subject to
the following restrictions:

 1. The origin of this software must not be misrepresented; you must not claim
    that you wrote the original software. If you use this software in a product,
    an acknowledgment in the product documentation would be appreciated but is
    not required.

 2. Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.

 3. This notice may not be removed or altered from any source distribution.
</pre>
            </li>
        </ul>
    </main>
</body>

</html>

