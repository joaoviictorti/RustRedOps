(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32 i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;9;) (func))
  (type (;10;) (func (result i32)))
  (func (;0;) (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 8
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 245
                  i32.ge_u
                  if  ;; label = @8
                    local.get 0
                    i32.const -65587
                    i32.ge_u
                    br_if 7 (;@1;)
                    local.get 0
                    i32.const 11
                    i32.add
                    local.tee 0
                    i32.const -8
                    i32.and
                    local.set 5
                    i32.const 1050004
                    i32.load
                    local.tee 9
                    i32.eqz
                    br_if 4 (;@4;)
                    i32.const 0
                    local.get 5
                    i32.sub
                    local.set 3
                    block (result i32)  ;; label = @9
                      i32.const 0
                      local.get 5
                      i32.const 256
                      i32.lt_u
                      br_if 0 (;@9;)
                      drop
                      i32.const 31
                      local.get 5
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      drop
                      local.get 5
                      i32.const 6
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee 0
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                    end
                    local.tee 7
                    i32.const 2
                    i32.shl
                    i32.const 1049592
                    i32.add
                    i32.load
                    local.tee 1
                    i32.eqz
                    if  ;; label = @9
                      i32.const 0
                      local.set 0
                      br 2 (;@7;)
                    end
                    i32.const 0
                    local.set 0
                    local.get 5
                    i32.const 25
                    local.get 7
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    i32.const 0
                    local.get 7
                    i32.const 31
                    i32.ne
                    select
                    i32.shl
                    local.set 4
                    loop  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.tee 6
                        local.get 5
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 5
                        i32.sub
                        local.tee 6
                        local.get 3
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.set 2
                        local.get 6
                        local.tee 3
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 3
                        local.get 1
                        local.set 0
                        br 4 (;@6;)
                      end
                      local.get 1
                      i32.const 20
                      i32.add
                      i32.load
                      local.tee 6
                      local.get 0
                      local.get 6
                      local.get 1
                      local.get 4
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      i32.load
                      local.tee 1
                      i32.ne
                      select
                      local.get 0
                      local.get 6
                      select
                      local.set 0
                      local.get 4
                      i32.const 1
                      i32.shl
                      local.set 4
                      local.get 1
                      br_if 0 (;@9;)
                    end
                    br 1 (;@7;)
                  end
                  i32.const 1050000
                  i32.load
                  local.tee 2
                  i32.const 16
                  local.get 0
                  i32.const 11
                  i32.add
                  i32.const -8
                  i32.and
                  local.get 0
                  i32.const 11
                  i32.lt_u
                  select
                  local.tee 5
                  i32.const 3
                  i32.shr_u
                  local.tee 0
                  i32.shr_u
                  local.tee 1
                  i32.const 3
                  i32.and
                  if  ;; label = @8
                    block  ;; label = @9
                      local.get 1
                      i32.const -1
                      i32.xor
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.add
                      local.tee 1
                      i32.const 3
                      i32.shl
                      local.tee 0
                      i32.const 1049736
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 1049744
                      i32.add
                      i32.load
                      local.tee 0
                      i32.load offset=8
                      local.tee 3
                      i32.ne
                      if  ;; label = @10
                        local.get 3
                        local.get 4
                        i32.store offset=12
                        local.get 4
                        local.get 3
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      i32.const 1050000
                      local.get 2
                      i32.const -2
                      local.get 1
                      i32.rotl
                      i32.and
                      i32.store
                    end
                    local.get 0
                    i32.const 8
                    i32.add
                    local.set 3
                    local.get 0
                    local.get 1
                    i32.const 3
                    i32.shl
                    local.tee 1
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 1
                    i32.add
                    local.tee 0
                    local.get 0
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    br 7 (;@1;)
                  end
                  local.get 5
                  i32.const 1050008
                  i32.load
                  i32.le_u
                  br_if 3 (;@4;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 1
                      i32.eqz
                      if  ;; label = @10
                        i32.const 1050004
                        i32.load
                        local.tee 0
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 0
                        i32.ctz
                        i32.const 2
                        i32.shl
                        i32.const 1049592
                        i32.add
                        i32.load
                        local.tee 1
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 5
                        i32.sub
                        local.set 3
                        local.get 1
                        local.set 2
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 1
                            i32.load offset=16
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 1
                            i32.const 20
                            i32.add
                            i32.load
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 2
                            i32.load offset=24
                            local.set 7
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 2
                                local.get 2
                                i32.load offset=12
                                local.tee 0
                                i32.eq
                                if  ;; label = @15
                                  local.get 2
                                  i32.const 20
                                  i32.const 16
                                  local.get 2
                                  i32.const 20
                                  i32.add
                                  local.tee 0
                                  i32.load
                                  local.tee 4
                                  select
                                  i32.add
                                  i32.load
                                  local.tee 1
                                  br_if 1 (;@14;)
                                  i32.const 0
                                  local.set 0
                                  br 2 (;@13;)
                                end
                                local.get 2
                                i32.load offset=8
                                local.tee 1
                                local.get 0
                                i32.store offset=12
                                local.get 0
                                local.get 1
                                i32.store offset=8
                                br 1 (;@13;)
                              end
                              local.get 0
                              local.get 2
                              i32.const 16
                              i32.add
                              local.get 4
                              select
                              local.set 4
                              loop  ;; label = @14
                                local.get 4
                                local.set 6
                                local.get 1
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.tee 1
                                local.get 0
                                i32.const 16
                                i32.add
                                local.get 1
                                i32.load
                                local.tee 1
                                select
                                local.set 4
                                local.get 0
                                i32.const 20
                                i32.const 16
                                local.get 1
                                select
                                i32.add
                                i32.load
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                              local.get 6
                              i32.const 0
                              i32.store
                            end
                            local.get 7
                            i32.eqz
                            br_if 4 (;@8;)
                            local.get 2
                            local.get 2
                            i32.load offset=28
                            i32.const 2
                            i32.shl
                            i32.const 1049592
                            i32.add
                            local.tee 1
                            i32.load
                            i32.ne
                            if  ;; label = @13
                              local.get 7
                              i32.const 16
                              i32.const 20
                              local.get 7
                              i32.load offset=16
                              local.get 2
                              i32.eq
                              select
                              i32.add
                              local.get 0
                              i32.store
                              local.get 0
                              i32.eqz
                              br_if 5 (;@8;)
                              br 4 (;@9;)
                            end
                            local.get 1
                            local.get 0
                            i32.store
                            local.get 0
                            br_if 3 (;@9;)
                            i32.const 1050004
                            i32.const 1050004
                            i32.load
                            i32.const -2
                            local.get 2
                            i32.load offset=28
                            i32.rotl
                            i32.and
                            i32.store
                            br 4 (;@8;)
                          end
                          local.get 0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 5
                          i32.sub
                          local.tee 1
                          local.get 3
                          local.get 1
                          local.get 3
                          i32.lt_u
                          local.tee 1
                          select
                          local.set 3
                          local.get 0
                          local.get 2
                          local.get 1
                          select
                          local.set 2
                          local.get 0
                          local.set 1
                          br 0 (;@11;)
                        end
                        unreachable
                      end
                      block  ;; label = @10
                        i32.const 2
                        local.get 0
                        i32.shl
                        local.tee 4
                        i32.const 0
                        local.get 4
                        i32.sub
                        i32.or
                        local.get 1
                        local.get 0
                        i32.shl
                        i32.and
                        i32.ctz
                        local.tee 1
                        i32.const 3
                        i32.shl
                        local.tee 0
                        i32.const 1049736
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 1049744
                        i32.add
                        i32.load
                        local.tee 0
                        i32.load offset=8
                        local.tee 3
                        i32.ne
                        if  ;; label = @11
                          local.get 3
                          local.get 4
                          i32.store offset=12
                          local.get 4
                          local.get 3
                          i32.store offset=8
                          br 1 (;@10;)
                        end
                        i32.const 1050000
                        local.get 2
                        i32.const -2
                        local.get 1
                        i32.rotl
                        i32.and
                        i32.store
                      end
                      local.get 0
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 5
                      i32.add
                      local.tee 6
                      local.get 1
                      i32.const 3
                      i32.shl
                      local.tee 1
                      local.get 5
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 1
                      i32.add
                      local.get 4
                      i32.store
                      i32.const 1050008
                      i32.load
                      local.tee 3
                      if  ;; label = @10
                        local.get 3
                        i32.const -8
                        i32.and
                        i32.const 1049736
                        i32.add
                        local.set 1
                        i32.const 1050016
                        i32.load
                        local.set 2
                        block (result i32)  ;; label = @11
                          i32.const 1050000
                          i32.load
                          local.tee 5
                          i32.const 1
                          local.get 3
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 3
                          i32.and
                          i32.eqz
                          if  ;; label = @12
                            i32.const 1050000
                            local.get 3
                            local.get 5
                            i32.or
                            i32.store
                            local.get 1
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                        end
                        local.set 3
                        local.get 1
                        local.get 2
                        i32.store offset=8
                        local.get 3
                        local.get 2
                        i32.store offset=12
                        local.get 2
                        local.get 1
                        i32.store offset=12
                        local.get 2
                        local.get 3
                        i32.store offset=8
                      end
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 3
                      i32.const 1050016
                      local.get 6
                      i32.store
                      i32.const 1050008
                      local.get 4
                      i32.store
                      br 8 (;@1;)
                    end
                    local.get 0
                    local.get 7
                    i32.store offset=24
                    local.get 2
                    i32.load offset=16
                    local.tee 1
                    if  ;; label = @9
                      local.get 0
                      local.get 1
                      i32.store offset=16
                      local.get 1
                      local.get 0
                      i32.store offset=24
                    end
                    local.get 2
                    i32.const 20
                    i32.add
                    i32.load
                    local.tee 1
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 20
                    i32.add
                    local.get 1
                    i32.store
                    local.get 1
                    local.get 0
                    i32.store offset=24
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      i32.const 16
                      i32.ge_u
                      if  ;; label = @10
                        local.get 2
                        local.get 5
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 2
                        local.get 5
                        i32.add
                        local.tee 4
                        local.get 3
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 3
                        local.get 4
                        i32.add
                        local.get 3
                        i32.store
                        i32.const 1050008
                        i32.load
                        local.tee 6
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 6
                        i32.const -8
                        i32.and
                        i32.const 1049736
                        i32.add
                        local.set 0
                        i32.const 1050016
                        i32.load
                        local.set 1
                        block (result i32)  ;; label = @11
                          i32.const 1050000
                          i32.load
                          local.tee 5
                          i32.const 1
                          local.get 6
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 6
                          i32.and
                          i32.eqz
                          if  ;; label = @12
                            i32.const 1050000
                            local.get 5
                            local.get 6
                            i32.or
                            i32.store
                            local.get 0
                            br 1 (;@11;)
                          end
                          local.get 0
                          i32.load offset=8
                        end
                        local.set 6
                        local.get 0
                        local.get 1
                        i32.store offset=8
                        local.get 6
                        local.get 1
                        i32.store offset=12
                        local.get 1
                        local.get 0
                        i32.store offset=12
                        local.get 1
                        local.get 6
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      local.get 2
                      local.get 3
                      local.get 5
                      i32.add
                      local.tee 0
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 2
                      i32.add
                      local.tee 0
                      local.get 0
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      br 1 (;@8;)
                    end
                    i32.const 1050016
                    local.get 4
                    i32.store
                    i32.const 1050008
                    local.get 3
                    i32.store
                  end
                  local.get 2
                  i32.const 8
                  i32.add
                  local.set 3
                  br 6 (;@1;)
                end
                local.get 0
                local.get 2
                i32.or
                i32.eqz
                if  ;; label = @7
                  i32.const 0
                  local.set 2
                  i32.const 2
                  local.get 7
                  i32.shl
                  local.tee 0
                  i32.const 0
                  local.get 0
                  i32.sub
                  i32.or
                  local.get 9
                  i32.and
                  local.tee 0
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.const 1049592
                  i32.add
                  i32.load
                  local.set 0
                end
                local.get 0
                i32.eqz
                br_if 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 0
                local.get 2
                local.get 0
                i32.load offset=4
                i32.const -8
                i32.and
                local.tee 4
                local.get 5
                i32.sub
                local.tee 6
                local.get 3
                i32.lt_u
                local.tee 7
                select
                local.set 9
                local.get 0
                i32.load offset=16
                local.tee 1
                i32.eqz
                if  ;; label = @7
                  local.get 0
                  i32.const 20
                  i32.add
                  i32.load
                  local.set 1
                end
                local.get 2
                local.get 9
                local.get 4
                local.get 5
                i32.lt_u
                local.tee 0
                select
                local.set 2
                local.get 3
                local.get 6
                local.get 3
                local.get 7
                select
                local.get 0
                select
                local.set 3
                local.get 1
                local.tee 0
                br_if 0 (;@6;)
              end
            end
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 1050008
            i32.load
            local.tee 0
            i32.le_u
            local.get 3
            local.get 0
            local.get 5
            i32.sub
            i32.ge_u
            i32.and
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                local.get 2
                i32.load offset=12
                local.tee 0
                i32.eq
                if  ;; label = @7
                  local.get 2
                  i32.const 20
                  i32.const 16
                  local.get 2
                  i32.const 20
                  i32.add
                  local.tee 0
                  i32.load
                  local.tee 4
                  select
                  i32.add
                  i32.load
                  local.tee 1
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 0
                  br 2 (;@5;)
                end
                local.get 2
                i32.load offset=8
                local.tee 1
                local.get 0
                i32.store offset=12
                local.get 0
                local.get 1
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 0
              local.get 2
              i32.const 16
              i32.add
              local.get 4
              select
              local.set 4
              loop  ;; label = @6
                local.get 4
                local.set 6
                local.get 1
                local.tee 0
                i32.const 20
                i32.add
                local.tee 1
                local.get 0
                i32.const 16
                i32.add
                local.get 1
                i32.load
                local.tee 1
                select
                local.set 4
                local.get 0
                i32.const 20
                i32.const 16
                local.get 1
                select
                i32.add
                i32.load
                local.tee 1
                br_if 0 (;@6;)
              end
              local.get 6
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            local.get 2
            i32.load offset=28
            i32.const 2
            i32.shl
            i32.const 1049592
            i32.add
            local.tee 1
            i32.load
            i32.ne
            if  ;; label = @5
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 2
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 3 (;@2;)
              br 2 (;@3;)
            end
            local.get 1
            local.get 0
            i32.store
            local.get 0
            br_if 1 (;@3;)
            i32.const 1050004
            i32.const 1050004
            i32.load
            i32.const -2
            local.get 2
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store
            br 2 (;@2;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 5
                    i32.const 1050008
                    i32.load
                    local.tee 1
                    i32.gt_u
                    if  ;; label = @9
                      local.get 5
                      i32.const 1050012
                      i32.load
                      local.tee 0
                      i32.ge_u
                      if  ;; label = @10
                        local.get 5
                        i32.const 65583
                        i32.add
                        i32.const -65536
                        i32.and
                        local.tee 2
                        i32.const 16
                        i32.shr_u
                        memory.grow
                        local.set 0
                        local.get 8
                        i32.const 4
                        i32.add
                        local.tee 1
                        i32.const 0
                        i32.store offset=8
                        local.get 1
                        i32.const 0
                        local.get 2
                        i32.const -65536
                        i32.and
                        local.get 0
                        i32.const -1
                        i32.eq
                        local.tee 2
                        select
                        i32.store offset=4
                        local.get 1
                        i32.const 0
                        local.get 0
                        i32.const 16
                        i32.shl
                        local.get 2
                        select
                        i32.store
                        local.get 8
                        i32.load offset=4
                        local.tee 1
                        i32.eqz
                        if  ;; label = @11
                          i32.const 0
                          local.set 3
                          br 10 (;@1;)
                        end
                        local.get 8
                        i32.load offset=12
                        local.set 6
                        i32.const 1050024
                        local.get 8
                        i32.load offset=8
                        local.tee 3
                        i32.const 1050024
                        i32.load
                        i32.add
                        local.tee 0
                        i32.store
                        i32.const 1050028
                        i32.const 1050028
                        i32.load
                        local.tee 2
                        local.get 0
                        local.get 0
                        local.get 2
                        i32.lt_u
                        select
                        i32.store
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 1050020
                            i32.load
                            local.tee 2
                            if  ;; label = @13
                              i32.const 1049720
                              local.set 0
                              loop  ;; label = @14
                                local.get 1
                                local.get 0
                                i32.load
                                local.tee 4
                                local.get 0
                                i32.load offset=4
                                local.tee 7
                                i32.add
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                              end
                              br 2 (;@11;)
                            end
                            i32.const 1050036
                            i32.load
                            local.tee 0
                            i32.const 0
                            local.get 0
                            local.get 1
                            i32.le_u
                            select
                            i32.eqz
                            if  ;; label = @13
                              i32.const 1050036
                              local.get 1
                              i32.store
                            end
                            i32.const 1050040
                            i32.const 4095
                            i32.store
                            i32.const 1049732
                            local.get 6
                            i32.store
                            i32.const 1049724
                            local.get 3
                            i32.store
                            i32.const 1049720
                            local.get 1
                            i32.store
                            i32.const 1049748
                            i32.const 1049736
                            i32.store
                            i32.const 1049756
                            i32.const 1049744
                            i32.store
                            i32.const 1049744
                            i32.const 1049736
                            i32.store
                            i32.const 1049764
                            i32.const 1049752
                            i32.store
                            i32.const 1049752
                            i32.const 1049744
                            i32.store
                            i32.const 1049772
                            i32.const 1049760
                            i32.store
                            i32.const 1049760
                            i32.const 1049752
                            i32.store
                            i32.const 1049780
                            i32.const 1049768
                            i32.store
                            i32.const 1049768
                            i32.const 1049760
                            i32.store
                            i32.const 1049788
                            i32.const 1049776
                            i32.store
                            i32.const 1049776
                            i32.const 1049768
                            i32.store
                            i32.const 1049796
                            i32.const 1049784
                            i32.store
                            i32.const 1049784
                            i32.const 1049776
                            i32.store
                            i32.const 1049804
                            i32.const 1049792
                            i32.store
                            i32.const 1049792
                            i32.const 1049784
                            i32.store
                            i32.const 1049812
                            i32.const 1049800
                            i32.store
                            i32.const 1049800
                            i32.const 1049792
                            i32.store
                            i32.const 1049808
                            i32.const 1049800
                            i32.store
                            i32.const 1049820
                            i32.const 1049808
                            i32.store
                            i32.const 1049816
                            i32.const 1049808
                            i32.store
                            i32.const 1049828
                            i32.const 1049816
                            i32.store
                            i32.const 1049824
                            i32.const 1049816
                            i32.store
                            i32.const 1049836
                            i32.const 1049824
                            i32.store
                            i32.const 1049832
                            i32.const 1049824
                            i32.store
                            i32.const 1049844
                            i32.const 1049832
                            i32.store
                            i32.const 1049840
                            i32.const 1049832
                            i32.store
                            i32.const 1049852
                            i32.const 1049840
                            i32.store
                            i32.const 1049848
                            i32.const 1049840
                            i32.store
                            i32.const 1049860
                            i32.const 1049848
                            i32.store
                            i32.const 1049856
                            i32.const 1049848
                            i32.store
                            i32.const 1049868
                            i32.const 1049856
                            i32.store
                            i32.const 1049864
                            i32.const 1049856
                            i32.store
                            i32.const 1049876
                            i32.const 1049864
                            i32.store
                            i32.const 1049884
                            i32.const 1049872
                            i32.store
                            i32.const 1049872
                            i32.const 1049864
                            i32.store
                            i32.const 1049892
                            i32.const 1049880
                            i32.store
                            i32.const 1049880
                            i32.const 1049872
                            i32.store
                            i32.const 1049900
                            i32.const 1049888
                            i32.store
                            i32.const 1049888
                            i32.const 1049880
                            i32.store
                            i32.const 1049908
                            i32.const 1049896
                            i32.store
                            i32.const 1049896
                            i32.const 1049888
                            i32.store
                            i32.const 1049916
                            i32.const 1049904
                            i32.store
                            i32.const 1049904
                            i32.const 1049896
                            i32.store
                            i32.const 1049924
                            i32.const 1049912
                            i32.store
                            i32.const 1049912
                            i32.const 1049904
                            i32.store
                            i32.const 1049932
                            i32.const 1049920
                            i32.store
                            i32.const 1049920
                            i32.const 1049912
                            i32.store
                            i32.const 1049940
                            i32.const 1049928
                            i32.store
                            i32.const 1049928
                            i32.const 1049920
                            i32.store
                            i32.const 1049948
                            i32.const 1049936
                            i32.store
                            i32.const 1049936
                            i32.const 1049928
                            i32.store
                            i32.const 1049956
                            i32.const 1049944
                            i32.store
                            i32.const 1049944
                            i32.const 1049936
                            i32.store
                            i32.const 1049964
                            i32.const 1049952
                            i32.store
                            i32.const 1049952
                            i32.const 1049944
                            i32.store
                            i32.const 1049972
                            i32.const 1049960
                            i32.store
                            i32.const 1049960
                            i32.const 1049952
                            i32.store
                            i32.const 1049980
                            i32.const 1049968
                            i32.store
                            i32.const 1049968
                            i32.const 1049960
                            i32.store
                            i32.const 1049988
                            i32.const 1049976
                            i32.store
                            i32.const 1049976
                            i32.const 1049968
                            i32.store
                            i32.const 1049996
                            i32.const 1049984
                            i32.store
                            i32.const 1049984
                            i32.const 1049976
                            i32.store
                            i32.const 1050020
                            local.get 1
                            i32.const 15
                            i32.add
                            i32.const -8
                            i32.and
                            local.tee 0
                            i32.const 8
                            i32.sub
                            local.tee 2
                            i32.store
                            i32.const 1049992
                            i32.const 1049984
                            i32.store
                            i32.const 1050012
                            local.get 3
                            i32.const 40
                            i32.sub
                            local.tee 4
                            local.get 1
                            local.get 0
                            i32.sub
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 0
                            i32.store
                            local.get 2
                            local.get 0
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 1
                            local.get 4
                            i32.add
                            i32.const 40
                            i32.store offset=4
                            i32.const 1050032
                            i32.const 2097152
                            i32.store
                            br 8 (;@4;)
                          end
                          local.get 2
                          local.get 4
                          i32.lt_u
                          local.get 1
                          local.get 2
                          i32.le_u
                          i32.or
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load offset=12
                          local.tee 4
                          i32.const 1
                          i32.and
                          br_if 0 (;@11;)
                          local.get 4
                          i32.const 1
                          i32.shr_u
                          local.get 6
                          i32.eq
                          br_if 3 (;@8;)
                        end
                        i32.const 1050036
                        i32.const 1050036
                        i32.load
                        local.tee 0
                        local.get 1
                        local.get 0
                        local.get 1
                        i32.lt_u
                        select
                        i32.store
                        local.get 1
                        local.get 3
                        i32.add
                        local.set 4
                        i32.const 1049720
                        local.set 0
                        block  ;; label = @11
                          block  ;; label = @12
                            loop  ;; label = @13
                              local.get 4
                              local.get 0
                              i32.load
                              i32.ne
                              if  ;; label = @14
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 1 (;@13;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 0
                            i32.load offset=12
                            local.tee 7
                            i32.const 1
                            i32.and
                            br_if 0 (;@12;)
                            local.get 7
                            i32.const 1
                            i32.shr_u
                            local.get 6
                            i32.eq
                            br_if 1 (;@11;)
                          end
                          i32.const 1049720
                          local.set 0
                          loop  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              local.get 0
                              i32.load
                              local.tee 4
                              i32.ge_u
                              if  ;; label = @14
                                local.get 4
                                local.get 0
                                i32.load offset=4
                                i32.add
                                local.tee 7
                                local.get 2
                                i32.gt_u
                                br_if 1 (;@13;)
                              end
                              local.get 0
                              i32.load offset=8
                              local.set 0
                              br 1 (;@12;)
                            end
                          end
                          i32.const 1050020
                          local.get 1
                          i32.const 15
                          i32.add
                          i32.const -8
                          i32.and
                          local.tee 0
                          i32.const 8
                          i32.sub
                          local.tee 4
                          i32.store
                          i32.const 1050012
                          local.get 3
                          i32.const 40
                          i32.sub
                          local.tee 9
                          local.get 1
                          local.get 0
                          i32.sub
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 0
                          i32.store
                          local.get 4
                          local.get 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 1
                          local.get 9
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          i32.const 1050032
                          i32.const 2097152
                          i32.store
                          local.get 2
                          local.get 7
                          i32.const 32
                          i32.sub
                          i32.const -8
                          i32.and
                          i32.const 8
                          i32.sub
                          local.tee 0
                          local.get 0
                          local.get 2
                          i32.const 16
                          i32.add
                          i32.lt_u
                          select
                          local.tee 4
                          i32.const 27
                          i32.store offset=4
                          i32.const 1049720
                          i64.load align=4
                          local.set 10
                          local.get 4
                          i32.const 16
                          i32.add
                          i32.const 1049728
                          i64.load align=4
                          i64.store align=4
                          local.get 4
                          local.get 10
                          i64.store offset=8 align=4
                          i32.const 1049732
                          local.get 6
                          i32.store
                          i32.const 1049724
                          local.get 3
                          i32.store
                          i32.const 1049720
                          local.get 1
                          i32.store
                          i32.const 1049728
                          local.get 4
                          i32.const 8
                          i32.add
                          i32.store
                          local.get 4
                          i32.const 28
                          i32.add
                          local.set 0
                          loop  ;; label = @12
                            local.get 0
                            i32.const 7
                            i32.store
                            local.get 0
                            i32.const 4
                            i32.add
                            local.tee 0
                            local.get 7
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          local.get 2
                          local.get 4
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 4
                          local.get 4
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                          local.get 2
                          local.get 4
                          local.get 2
                          i32.sub
                          local.tee 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 4
                          local.get 0
                          i32.store
                          local.get 0
                          i32.const 256
                          i32.ge_u
                          if  ;; label = @12
                            local.get 2
                            local.get 0
                            call 7
                            br 8 (;@4;)
                          end
                          local.get 0
                          i32.const -8
                          i32.and
                          i32.const 1049736
                          i32.add
                          local.set 1
                          block (result i32)  ;; label = @12
                            i32.const 1050000
                            i32.load
                            local.tee 4
                            i32.const 1
                            local.get 0
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 0
                            i32.and
                            i32.eqz
                            if  ;; label = @13
                              i32.const 1050000
                              local.get 0
                              local.get 4
                              i32.or
                              i32.store
                              local.get 1
                              br 1 (;@12;)
                            end
                            local.get 1
                            i32.load offset=8
                          end
                          local.set 0
                          local.get 1
                          local.get 2
                          i32.store offset=8
                          local.get 0
                          local.get 2
                          i32.store offset=12
                          local.get 2
                          local.get 1
                          i32.store offset=12
                          local.get 2
                          local.get 0
                          i32.store offset=8
                          br 7 (;@4;)
                        end
                        local.get 0
                        local.get 1
                        i32.store
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        local.get 3
                        i32.add
                        i32.store offset=4
                        local.get 1
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const 8
                        i32.sub
                        local.tee 2
                        local.get 5
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 4
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const 8
                        i32.sub
                        local.tee 3
                        local.get 2
                        local.get 5
                        i32.add
                        local.tee 0
                        i32.sub
                        local.set 5
                        local.get 3
                        i32.const 1050020
                        i32.load
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 3
                        i32.const 1050016
                        i32.load
                        i32.eq
                        br_if 4 (;@6;)
                        local.get 3
                        i32.load offset=4
                        local.tee 1
                        i32.const 3
                        i32.and
                        i32.const 1
                        i32.eq
                        if  ;; label = @11
                          local.get 3
                          local.get 1
                          i32.const -8
                          i32.and
                          local.tee 1
                          call 5
                          local.get 1
                          local.get 5
                          i32.add
                          local.set 5
                          local.get 1
                          local.get 3
                          i32.add
                          local.tee 3
                          i32.load offset=4
                          local.set 1
                        end
                        local.get 3
                        local.get 1
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 0
                        local.get 5
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 5
                        i32.add
                        local.get 5
                        i32.store
                        local.get 5
                        i32.const 256
                        i32.ge_u
                        if  ;; label = @11
                          local.get 0
                          local.get 5
                          call 7
                          br 6 (;@5;)
                        end
                        local.get 5
                        i32.const -8
                        i32.and
                        i32.const 1049736
                        i32.add
                        local.set 1
                        block (result i32)  ;; label = @11
                          i32.const 1050000
                          i32.load
                          local.tee 4
                          i32.const 1
                          local.get 5
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 3
                          i32.and
                          i32.eqz
                          if  ;; label = @12
                            i32.const 1050000
                            local.get 3
                            local.get 4
                            i32.or
                            i32.store
                            local.get 1
                            br 1 (;@11;)
                          end
                          local.get 1
                          i32.load offset=8
                        end
                        local.set 4
                        local.get 1
                        local.get 0
                        i32.store offset=8
                        local.get 4
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 1
                        i32.store offset=12
                        local.get 0
                        local.get 4
                        i32.store offset=8
                        br 5 (;@5;)
                      end
                      i32.const 1050012
                      local.get 0
                      local.get 5
                      i32.sub
                      local.tee 1
                      i32.store
                      i32.const 1050020
                      i32.const 1050020
                      i32.load
                      local.tee 0
                      local.get 5
                      i32.add
                      local.tee 2
                      i32.store
                      local.get 2
                      local.get 1
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 3
                      br 8 (;@1;)
                    end
                    i32.const 1050016
                    i32.load
                    local.set 0
                    block  ;; label = @9
                      local.get 1
                      local.get 5
                      i32.sub
                      local.tee 2
                      i32.const 15
                      i32.le_u
                      if  ;; label = @10
                        i32.const 1050016
                        i32.const 0
                        i32.store
                        i32.const 1050008
                        i32.const 0
                        i32.store
                        local.get 0
                        local.get 1
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 1
                        i32.add
                        local.tee 1
                        local.get 1
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 1 (;@9;)
                      end
                      i32.const 1050008
                      local.get 2
                      i32.store
                      i32.const 1050016
                      local.get 0
                      local.get 5
                      i32.add
                      local.tee 4
                      i32.store
                      local.get 4
                      local.get 2
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 1
                      i32.add
                      local.get 2
                      i32.store
                      local.get 0
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                    end
                    local.get 0
                    i32.const 8
                    i32.add
                    local.set 3
                    br 7 (;@1;)
                  end
                  local.get 0
                  local.get 3
                  local.get 7
                  i32.add
                  i32.store offset=4
                  i32.const 1050020
                  i32.const 1050020
                  i32.load
                  local.tee 0
                  i32.const 15
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 1
                  i32.const 8
                  i32.sub
                  local.tee 2
                  i32.store
                  i32.const 1050012
                  i32.const 1050012
                  i32.load
                  local.get 3
                  i32.add
                  local.tee 4
                  local.get 0
                  local.get 1
                  i32.sub
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 2
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 4
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  i32.const 1050032
                  i32.const 2097152
                  i32.store
                  br 3 (;@4;)
                end
                i32.const 1050020
                local.get 0
                i32.store
                i32.const 1050012
                i32.const 1050012
                i32.load
                local.get 5
                i32.add
                local.tee 1
                i32.store
                local.get 0
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                br 1 (;@5;)
              end
              i32.const 1050016
              local.get 0
              i32.store
              i32.const 1050008
              i32.const 1050008
              i32.load
              local.get 5
              i32.add
              local.tee 1
              i32.store
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
            end
            local.get 2
            i32.const 8
            i32.add
            local.set 3
            br 3 (;@1;)
          end
          i32.const 0
          local.set 3
          i32.const 1050012
          i32.load
          local.tee 0
          local.get 5
          i32.le_u
          br_if 2 (;@1;)
          i32.const 1050012
          local.get 0
          local.get 5
          i32.sub
          local.tee 1
          i32.store
          i32.const 1050020
          i32.const 1050020
          i32.load
          local.tee 0
          local.get 5
          i32.add
          local.tee 2
          i32.store
          local.get 2
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.set 3
          br 2 (;@1;)
        end
        local.get 0
        local.get 7
        i32.store offset=24
        local.get 2
        i32.load offset=16
        local.tee 1
        if  ;; label = @3
          local.get 0
          local.get 1
          i32.store offset=16
          local.get 1
          local.get 0
          i32.store offset=24
        end
        local.get 2
        i32.const 20
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 20
        i32.add
        local.get 1
        i32.store
        local.get 1
        local.get 0
        i32.store offset=24
      end
      block  ;; label = @2
        local.get 3
        i32.const 16
        i32.ge_u
        if  ;; label = @3
          local.get 2
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 2
          local.get 5
          i32.add
          local.tee 0
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.get 3
          i32.store
          local.get 3
          i32.const 256
          i32.ge_u
          if  ;; label = @4
            local.get 0
            local.get 3
            call 7
            br 2 (;@2;)
          end
          local.get 3
          i32.const -8
          i32.and
          i32.const 1049736
          i32.add
          local.set 1
          block (result i32)  ;; label = @4
            i32.const 1050000
            i32.load
            local.tee 4
            i32.const 1
            local.get 3
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 3
            i32.and
            i32.eqz
            if  ;; label = @5
              i32.const 1050000
              local.get 3
              local.get 4
              i32.or
              i32.store
              local.get 1
              br 1 (;@4;)
            end
            local.get 1
            i32.load offset=8
          end
          local.set 4
          local.get 1
          local.get 0
          i32.store offset=8
          local.get 4
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=12
          local.get 0
          local.get 4
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 2
        local.get 3
        local.get 5
        i32.add
        local.tee 0
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 0
        local.get 0
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
      end
      local.get 2
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 8
    i32.const 16
    i32.add
    global.set 0
    local.get 3)
  (func (;1;) (type 3) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const 8
    i32.sub
    local.tee 1
    local.get 0
    i32.const 4
    i32.sub
    i32.load
    local.tee 3
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 3
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 1
            i32.load
            local.tee 3
            local.get 0
            i32.add
            local.set 0
            local.get 1
            local.get 3
            i32.sub
            local.tee 1
            i32.const 1050016
            i32.load
            i32.eq
            if  ;; label = @5
              local.get 2
              i32.load offset=4
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 1 (;@4;)
              i32.const 1050008
              local.get 0
              i32.store
              local.get 2
              local.get 2
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 0
              i32.store
              return
            end
            local.get 1
            local.get 3
            call 5
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 2
              i32.and
              i32.eqz
              if  ;; label = @6
                local.get 2
                i32.const 1050020
                i32.load
                i32.eq
                br_if 2 (;@4;)
                local.get 2
                i32.const 1050016
                i32.load
                i32.eq
                br_if 5 (;@1;)
                local.get 2
                local.get 3
                i32.const -8
                i32.and
                local.tee 2
                call 5
                local.get 1
                local.get 0
                local.get 2
                i32.add
                local.tee 0
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 0
                local.get 1
                i32.add
                local.get 0
                i32.store
                local.get 1
                i32.const 1050016
                i32.load
                i32.ne
                br_if 1 (;@5;)
                i32.const 1050008
                local.get 0
                i32.store
                return
              end
              local.get 2
              local.get 3
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 0
              i32.store
            end
            local.get 0
            i32.const 256
            i32.lt_u
            br_if 2 (;@2;)
            local.get 1
            local.get 0
            call 7
            i32.const 0
            local.set 1
            i32.const 1050040
            i32.const 1050040
            i32.load
            i32.const 1
            i32.sub
            local.tee 0
            i32.store
            local.get 0
            br_if 1 (;@3;)
            i32.const 1049728
            i32.load
            local.tee 0
            if  ;; label = @5
              loop  ;; label = @6
                local.get 1
                i32.const 1
                i32.add
                local.set 1
                local.get 0
                i32.load offset=8
                local.tee 0
                br_if 0 (;@6;)
              end
            end
            i32.const 1050040
            i32.const 4095
            local.get 1
            local.get 1
            i32.const 4095
            i32.le_u
            select
            i32.store
            return
          end
          i32.const 1050020
          local.get 1
          i32.store
          i32.const 1050012
          i32.const 1050012
          i32.load
          local.get 0
          i32.add
          local.tee 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          i32.const 1050016
          i32.load
          local.get 1
          i32.eq
          if  ;; label = @4
            i32.const 1050008
            i32.const 0
            i32.store
            i32.const 1050016
            i32.const 0
            i32.store
          end
          local.get 0
          i32.const 1050032
          i32.load
          local.tee 3
          i32.le_u
          br_if 0 (;@3;)
          i32.const 1050020
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          block  ;; label = @4
            i32.const 1050012
            i32.load
            local.tee 4
            i32.const 41
            i32.lt_u
            br_if 0 (;@4;)
            i32.const 1049720
            local.set 0
            loop  ;; label = @5
              local.get 2
              local.get 0
              i32.load
              local.tee 5
              i32.ge_u
              if  ;; label = @6
                local.get 5
                local.get 0
                i32.load offset=4
                i32.add
                local.get 2
                i32.gt_u
                br_if 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 0
              br_if 0 (;@5;)
            end
          end
          i32.const 1049728
          i32.load
          local.tee 0
          if  ;; label = @4
            loop  ;; label = @5
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 0
              i32.load offset=8
              local.tee 0
              br_if 0 (;@5;)
            end
          end
          i32.const 1050040
          i32.const 4095
          local.get 1
          local.get 1
          i32.const 4095
          i32.le_u
          select
          i32.store
          local.get 3
          local.get 4
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 1050032
          i32.const -1
          i32.store
        end
        return
      end
      local.get 0
      i32.const -8
      i32.and
      i32.const 1049736
      i32.add
      local.set 2
      block (result i32)  ;; label = @2
        i32.const 1050000
        i32.load
        local.tee 3
        i32.const 1
        local.get 0
        i32.const 3
        i32.shr_u
        i32.shl
        local.tee 0
        i32.and
        i32.eqz
        if  ;; label = @3
          i32.const 1050000
          local.get 0
          local.get 3
          i32.or
          i32.store
          local.get 2
          br 1 (;@2;)
        end
        local.get 2
        i32.load offset=8
      end
      local.set 0
      local.get 2
      local.get 1
      i32.store offset=8
      local.get 0
      local.get 1
      i32.store offset=12
      local.get 1
      local.get 2
      i32.store offset=12
      local.get 1
      local.get 0
      i32.store offset=8
      return
    end
    i32.const 1050016
    local.get 1
    i32.store
    i32.const 1050008
    i32.const 1050008
    i32.load
    local.get 0
    i32.add
    local.tee 0
    i32.store
    local.get 1
    local.get 0
    i32.const 1
    i32.or
    i32.store offset=4
    local.get 0
    local.get 1
    i32.add
    local.get 0
    i32.store)
  (func (;2;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 36
    i32.add
    i32.const 1048928
    i32.store
    local.get 2
    i32.const 3
    i32.store8 offset=44
    local.get 2
    i32.const 32
    i32.store offset=28
    local.get 2
    i32.const 0
    i32.store offset=40
    local.get 2
    local.get 0
    i32.store offset=32
    local.get 2
    i32.const 0
    i32.store offset=20
    local.get 2
    i32.const 0
    i32.store offset=12
    block (result i32)  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.load offset=16
            local.tee 10
            i32.eqz
            if  ;; label = @5
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.tee 0
              i32.eqz
              br_if 1 (;@4;)
              local.get 1
              i32.load offset=8
              local.tee 3
              local.get 0
              i32.const 3
              i32.shl
              i32.add
              local.set 4
              local.get 0
              i32.const 1
              i32.sub
              i32.const 536870911
              i32.and
              i32.const 1
              i32.add
              local.set 7
              local.get 1
              i32.load
              local.set 0
              loop  ;; label = @6
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 5
                if  ;; label = @7
                  local.get 2
                  i32.load offset=32
                  local.get 0
                  i32.load
                  local.get 5
                  local.get 2
                  i32.load offset=36
                  i32.load offset=12
                  call_indirect (type 2)
                  br_if 4 (;@3;)
                end
                local.get 3
                i32.load
                local.get 2
                i32.const 12
                i32.add
                local.get 3
                i32.const 4
                i32.add
                i32.load
                call_indirect (type 1)
                br_if 3 (;@3;)
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                local.get 3
                i32.const 8
                i32.add
                local.tee 3
                local.get 4
                i32.ne
                br_if 0 (;@6;)
              end
              br 1 (;@4;)
            end
            local.get 1
            i32.const 20
            i32.add
            i32.load
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 5
            i32.shl
            local.set 11
            local.get 0
            i32.const 1
            i32.sub
            i32.const 134217727
            i32.and
            i32.const 1
            i32.add
            local.set 7
            local.get 1
            i32.load offset=8
            local.set 5
            local.get 1
            i32.load
            local.set 0
            loop  ;; label = @5
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 3
              if  ;; label = @6
                local.get 2
                i32.load offset=32
                local.get 0
                i32.load
                local.get 3
                local.get 2
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 2)
                br_if 3 (;@3;)
              end
              local.get 2
              local.get 8
              local.get 10
              i32.add
              local.tee 3
              i32.const 16
              i32.add
              i32.load
              i32.store offset=28
              local.get 2
              local.get 3
              i32.const 28
              i32.add
              i32.load8_u
              i32.store8 offset=44
              local.get 2
              local.get 3
              i32.const 24
              i32.add
              i32.load
              i32.store offset=40
              local.get 3
              i32.const 12
              i32.add
              i32.load
              local.set 6
              i32.const 0
              local.set 9
              i32.const 0
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    i32.const 8
                    i32.add
                    i32.load
                    i32.const 1
                    i32.sub
                    br_table 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 6
                  i32.const 3
                  i32.shl
                  local.get 5
                  i32.add
                  local.tee 12
                  i32.load offset=4
                  i32.const 17
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 12
                  i32.load
                  i32.load
                  local.set 6
                end
                i32.const 1
                local.set 4
              end
              local.get 2
              local.get 6
              i32.store offset=16
              local.get 2
              local.get 4
              i32.store offset=12
              local.get 3
              i32.const 4
              i32.add
              i32.load
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    i32.load
                    i32.const 1
                    i32.sub
                    br_table 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 4
                  i32.const 3
                  i32.shl
                  local.get 5
                  i32.add
                  local.tee 6
                  i32.load offset=4
                  i32.const 17
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 6
                  i32.load
                  i32.load
                  local.set 4
                end
                i32.const 1
                local.set 9
              end
              local.get 2
              local.get 4
              i32.store offset=24
              local.get 2
              local.get 9
              i32.store offset=20
              local.get 5
              local.get 3
              i32.const 20
              i32.add
              i32.load
              i32.const 3
              i32.shl
              i32.add
              local.tee 3
              i32.load
              local.get 2
              i32.const 12
              i32.add
              local.get 3
              i32.const 4
              i32.add
              i32.load
              call_indirect (type 1)
              br_if 2 (;@3;)
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 11
              local.get 8
              i32.const 32
              i32.add
              local.tee 8
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 7
          local.get 1
          i32.load offset=4
          i32.ge_u
          br_if 1 (;@2;)
          local.get 2
          i32.load offset=32
          local.get 1
          i32.load
          local.get 7
          i32.const 3
          i32.shl
          i32.add
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          local.get 2
          i32.load offset=36
          i32.load offset=12
          call_indirect (type 2)
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 1
        br 1 (;@1;)
      end
      i32.const 0
    end
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func (;3;) (type 0) (param i32 i32)
    (local i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        local.get 0
        local.get 3
        i32.sub
        local.tee 0
        i32.const 1050016
        i32.load
        i32.eq
        if  ;; label = @3
          local.get 2
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 1050008
          local.get 1
          i32.store
          local.get 2
          local.get 2
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          i32.store
          return
        end
        local.get 0
        local.get 3
        call 5
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.const 2
            i32.and
            i32.eqz
            if  ;; label = @5
              local.get 2
              i32.const 1050020
              i32.load
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 1050016
              i32.load
              i32.eq
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const -8
              i32.and
              local.tee 2
              call 5
              local.get 0
              local.get 1
              local.get 2
              i32.add
              local.tee 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              i32.const 1050016
              i32.load
              i32.ne
              br_if 1 (;@4;)
              i32.const 1050008
              local.get 1
              i32.store
              return
            end
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
          end
          local.get 1
          i32.const 256
          i32.ge_u
          if  ;; label = @4
            local.get 0
            local.get 1
            call 7
            br 3 (;@1;)
          end
          local.get 1
          i32.const -8
          i32.and
          i32.const 1049736
          i32.add
          local.set 2
          block (result i32)  ;; label = @4
            i32.const 1050000
            i32.load
            local.tee 3
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 1
            i32.and
            i32.eqz
            if  ;; label = @5
              i32.const 1050000
              local.get 1
              local.get 3
              i32.or
              i32.store
              local.get 2
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
          end
          local.set 1
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 1050020
        local.get 0
        i32.store
        i32.const 1050012
        i32.const 1050012
        i32.load
        local.get 1
        i32.add
        local.tee 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 1050016
        i32.load
        i32.ne
        br_if 1 (;@1;)
        i32.const 1050008
        i32.const 0
        i32.store
        i32.const 1050016
        i32.const 0
        i32.store
        return
      end
      i32.const 1050016
      local.get 0
      i32.store
      i32.const 1050008
      i32.const 1050008
      i32.load
      local.get 1
      i32.add
      local.tee 1
      i32.store
      local.get 0
      local.get 1
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.add
      local.get 1
      i32.store
    end)
  (func (;4;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      i32.const -65587
      i32.const 16
      local.get 0
      local.get 0
      i32.const 16
      i32.le_u
      select
      local.tee 0
      i32.sub
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      local.get 1
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 4
      i32.add
      i32.const 12
      i32.add
      call 0
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 8
      i32.sub
      local.set 1
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.sub
        local.tee 3
        local.get 2
        i32.and
        i32.eqz
        if  ;; label = @3
          local.get 1
          local.set 0
          br 1 (;@2;)
        end
        local.get 2
        i32.const 4
        i32.sub
        local.tee 5
        i32.load
        local.tee 6
        i32.const -8
        i32.and
        local.get 2
        local.get 3
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const 8
        i32.sub
        local.tee 2
        local.get 0
        i32.const 0
        local.get 2
        local.get 1
        i32.sub
        i32.const 16
        i32.le_u
        select
        i32.add
        local.tee 0
        local.get 1
        i32.sub
        local.tee 2
        i32.sub
        local.set 3
        local.get 6
        i32.const 3
        i32.and
        if  ;; label = @3
          local.get 0
          local.get 3
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.tee 3
          local.get 3
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 2
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 1
          local.get 2
          i32.add
          local.tee 3
          local.get 3
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 2
          call 3
          br 1 (;@2;)
        end
        local.get 1
        i32.load
        local.set 1
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        local.get 1
        local.get 2
        i32.add
        i32.store
      end
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.tee 2
        local.get 4
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        local.get 1
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 4
        i32.add
        local.tee 1
        local.get 2
        local.get 4
        i32.sub
        local.tee 4
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 4
        call 3
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 3)
  (func (;5;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 256
        i32.ge_u
        if  ;; label = @3
          local.get 0
          i32.load offset=24
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 2
              i32.eq
              if  ;; label = @6
                local.get 0
                i32.const 20
                i32.const 16
                local.get 0
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                select
                i32.add
                i32.load
                local.tee 1
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 2
            local.get 0
            i32.const 16
            i32.add
            local.get 4
            select
            local.set 4
            loop  ;; label = @5
              local.get 4
              local.set 5
              local.get 1
              local.tee 2
              i32.const 20
              i32.add
              local.tee 1
              local.get 2
              i32.const 16
              i32.add
              local.get 1
              i32.load
              local.tee 1
              select
              local.set 4
              local.get 2
              i32.const 20
              i32.const 16
              local.get 1
              select
              i32.add
              i32.load
              local.tee 1
              br_if 0 (;@5;)
            end
            local.get 5
            i32.const 0
            i32.store
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 0
          i32.load offset=28
          i32.const 2
          i32.shl
          i32.const 1049592
          i32.add
          local.tee 1
          i32.load
          i32.ne
          if  ;; label = @4
            local.get 3
            i32.const 16
            i32.const 20
            local.get 3
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 2
            i32.store
            local.get 2
            i32.eqz
            br_if 3 (;@1;)
            br 2 (;@2;)
          end
          local.get 1
          local.get 2
          i32.store
          local.get 2
          br_if 1 (;@2;)
          i32.const 1050004
          i32.const 1050004
          i32.load
          i32.const -2
          local.get 0
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        i32.load offset=8
        local.tee 0
        local.get 2
        i32.ne
        if  ;; label = @3
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 2
          local.get 0
          i32.store offset=8
          return
        end
        i32.const 1050000
        i32.const 1050000
        i32.load
        i32.const -2
        local.get 1
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store
        return
      end
      local.get 2
      local.get 3
      i32.store offset=24
      local.get 0
      i32.load offset=16
      local.tee 1
      if  ;; label = @2
        local.get 2
        local.get 1
        i32.store offset=16
        local.get 1
        local.get 2
        i32.store offset=24
      end
      local.get 0
      i32.const 20
      i32.add
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 20
      i32.add
      local.get 0
      i32.store
      local.get 0
      local.get 2
      i32.store offset=24
    end)
  (func (;6;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 128
          i32.ge_u
          if  ;; label = @4
            local.get 3
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.lt_u
            if  ;; label = @5
              local.get 3
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 3
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 3
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              br 3 (;@2;)
            end
            local.get 3
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 3
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 3
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 3
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            br 2 (;@2;)
          end
          local.get 0
          i32.load offset=8
          local.tee 2
          local.get 0
          i32.load offset=4
          i32.eq
          if  ;; label = @4
            global.get 0
            i32.const 32
            i32.sub
            local.tee 4
            global.set 0
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 1
                i32.add
                local.tee 2
                i32.eqz
                br_if 0 (;@6;)
                i32.const 8
                local.get 0
                i32.load offset=4
                local.tee 6
                i32.const 1
                i32.shl
                local.tee 5
                local.get 2
                local.get 2
                local.get 5
                i32.lt_u
                select
                local.tee 2
                local.get 2
                i32.const 8
                i32.le_u
                select
                local.tee 5
                i32.const -1
                i32.xor
                i32.const 31
                i32.shr_u
                local.set 2
                block  ;; label = @7
                  local.get 6
                  i32.eqz
                  if  ;; label = @8
                    local.get 4
                    i32.const 0
                    i32.store offset=24
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 6
                  i32.store offset=28
                  local.get 4
                  i32.const 1
                  i32.store offset=24
                  local.get 4
                  local.get 0
                  i32.load
                  i32.store offset=20
                end
                local.get 4
                i32.const 8
                i32.add
                local.get 2
                local.get 5
                local.get 4
                i32.const 20
                i32.add
                call 11
                local.get 4
                i32.load offset=12
                local.set 2
                local.get 4
                i32.load offset=8
                i32.eqz
                if  ;; label = @7
                  local.get 0
                  local.get 5
                  i32.store offset=4
                  local.get 0
                  local.get 2
                  i32.store
                  br 2 (;@5;)
                end
                local.get 2
                i32.const -2147483647
                i32.eq
                br_if 1 (;@5;)
                local.get 2
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                local.get 4
                i32.const 16
                i32.add
                i32.load
                call 30
                unreachable
              end
              call 17
              unreachable
            end
            local.get 4
            i32.const 32
            i32.add
            global.set 0
            local.get 0
            i32.load offset=8
            local.set 2
          end
          local.get 0
          local.get 2
          i32.const 1
          i32.add
          i32.store offset=8
          local.get 0
          i32.load
          local.get 2
          i32.add
          local.get 1
          i32.store8
          br 2 (;@1;)
        end
        local.get 3
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 3
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
      end
      local.set 1
      local.get 1
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.sub
      i32.gt_u
      if  ;; label = @2
        local.get 0
        local.get 2
        local.get 1
        call 10
        local.get 0
        i32.load offset=8
        local.set 2
      end
      local.get 0
      i32.load
      local.get 2
      i32.add
      local.get 3
      i32.const 12
      i32.add
      local.get 1
      call 31
      drop
      local.get 0
      local.get 1
      local.get 2
      i32.add
      i32.store offset=8
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func (;7;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    block (result i32)  ;; label = @1
      i32.const 0
      local.get 1
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      drop
      i32.const 31
      local.get 1
      i32.const 16777215
      i32.gt_u
      br_if 0 (;@1;)
      drop
      local.get 1
      i32.const 6
      local.get 1
      i32.const 8
      i32.shr_u
      i32.clz
      local.tee 3
      i32.sub
      i32.shr_u
      i32.const 1
      i32.and
      local.get 3
      i32.const 1
      i32.shl
      i32.sub
      i32.const 62
      i32.add
    end
    local.tee 2
    i32.store offset=28
    local.get 2
    i32.const 2
    i32.shl
    i32.const 1049592
    i32.add
    local.set 4
    block  ;; label = @1
      i32.const 1050004
      i32.load
      local.tee 5
      i32.const 1
      local.get 2
      i32.shl
      local.tee 3
      i32.and
      i32.eqz
      if  ;; label = @2
        i32.const 1050004
        local.get 3
        local.get 5
        i32.or
        i32.store
        local.get 4
        local.get 0
        i32.store
        local.get 0
        local.get 4
        i32.store offset=24
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 4
          i32.load
          local.tee 3
          i32.load offset=4
          i32.const -8
          i32.and
          i32.eq
          if  ;; label = @4
            local.get 3
            local.set 2
            br 1 (;@3;)
          end
          local.get 1
          i32.const 25
          local.get 2
          i32.const 1
          i32.shr_u
          i32.sub
          i32.const 0
          local.get 2
          i32.const 31
          i32.ne
          select
          i32.shl
          local.set 4
          loop  ;; label = @4
            local.get 3
            local.get 4
            i32.const 29
            i32.shr_u
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 5
            i32.load
            local.tee 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 4
            i32.const 1
            i32.shl
            local.set 4
            local.get 2
            local.set 3
            local.get 2
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 1
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 2
        i32.load offset=8
        local.tee 1
        local.get 0
        i32.store offset=12
        local.get 2
        local.get 0
        i32.store offset=8
        local.get 0
        i32.const 0
        i32.store offset=24
        local.get 0
        local.get 2
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        return
      end
      local.get 5
      local.get 0
      i32.store
      local.get 0
      local.get 3
      i32.store offset=24
    end
    local.get 0
    local.get 0
    i32.store offset=12
    local.get 0
    local.get 0
    i32.store offset=8)
  (func (;8;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.set 4
    local.get 1
    i32.load offset=4
    i32.eqz
    if  ;; label = @1
      local.get 1
      i32.load
      local.set 3
      local.get 2
      i32.const 44
      i32.add
      local.tee 5
      i32.const 0
      i32.store
      local.get 2
      i64.const 1
      i64.store offset=36 align=4
      local.get 2
      i32.const 36
      i32.add
      local.get 3
      call 2
      drop
      local.get 2
      i32.const 32
      i32.add
      local.get 5
      i32.load
      local.tee 3
      i32.store
      local.get 2
      local.get 2
      i64.load offset=36 align=4
      local.tee 6
      i64.store offset=24
      local.get 4
      i32.const 8
      i32.add
      local.get 3
      i32.store
      local.get 4
      local.get 6
      i64.store align=4
    end
    local.get 2
    i32.const 16
    i32.add
    local.tee 3
    local.get 4
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 1
    i32.const 12
    i32.add
    i32.const 0
    i32.store
    local.get 4
    i64.load align=4
    local.set 6
    local.get 1
    i64.const 1
    i64.store offset=4 align=4
    i32.const 1049533
    i32.load8_u
    drop
    local.get 2
    local.get 6
    i64.store offset=8
    i32.const 12
    i32.const 4
    call 22
    local.tee 1
    i32.eqz
    if  ;; label = @1
      i32.const 4
      i32.const 12
      call 30
      unreachable
    end
    local.get 1
    local.get 2
    i64.load offset=8
    i64.store align=4
    local.get 1
    i32.const 8
    i32.add
    local.get 3
    i32.load
    i32.store
    local.get 0
    i32.const 1049088
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func (;9;) (type 5) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 6
    global.set 0
    i32.const 1049588
    i32.const 1049588
    i32.load
    local.tee 7
    i32.const 1
    i32.add
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        i32.const 1050048
        i32.load8_u
        br_if 0 (;@2;)
        i32.const 1050048
        i32.const 1
        i32.store8
        i32.const 1050044
        i32.const 1050044
        i32.load
        i32.const 1
        i32.add
        i32.store
        local.get 6
        local.get 5
        i32.store8 offset=29
        local.get 6
        local.get 4
        i32.store8 offset=28
        local.get 6
        local.get 3
        i32.store offset=24
        local.get 6
        local.get 2
        i32.store offset=20
        local.get 6
        i32.const 1049160
        i32.store offset=16
        local.get 6
        i32.const 1048884
        i32.store offset=12
        i32.const 1049572
        i32.load
        local.tee 2
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        i32.const 1049572
        local.get 2
        i32.const 1
        i32.add
        i32.store
        i32.const 1049572
        i32.const 1049580
        i32.load
        if (result i32)  ;; label = @3
          local.get 6
          local.get 0
          local.get 1
          i32.load offset=16
          call_indirect (type 0)
          local.get 6
          local.get 6
          i64.load
          i64.store offset=12 align=4
          i32.const 1049580
          i32.load
          local.get 6
          i32.const 12
          i32.add
          i32.const 1049584
          i32.load
          i32.load offset=20
          call_indirect (type 0)
          i32.const 1049572
          i32.load
          i32.const 1
          i32.sub
        else
          local.get 2
        end
        i32.store
        i32.const 1050048
        i32.const 0
        i32.store8
        local.get 4
        br_if 1 (;@1;)
      end
      unreachable
    end
    unreachable)
  (func (;10;) (type 6) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 1
        local.get 2
        i32.add
        local.tee 1
        i32.gt_u
        br_if 0 (;@2;)
        i32.const 8
        local.get 0
        i32.load offset=4
        local.tee 2
        i32.const 1
        i32.shl
        local.tee 4
        local.get 1
        local.get 1
        local.get 4
        i32.lt_u
        select
        local.tee 1
        local.get 1
        i32.const 8
        i32.le_u
        select
        local.tee 4
        i32.const -1
        i32.xor
        i32.const 31
        i32.shr_u
        local.set 1
        block  ;; label = @3
          local.get 2
          i32.eqz
          if  ;; label = @4
            local.get 3
            i32.const 0
            i32.store offset=24
            br 1 (;@3;)
          end
          local.get 3
          local.get 2
          i32.store offset=28
          local.get 3
          i32.const 1
          i32.store offset=24
          local.get 3
          local.get 0
          i32.load
          i32.store offset=20
        end
        local.get 3
        i32.const 8
        i32.add
        local.get 1
        local.get 4
        local.get 3
        i32.const 20
        i32.add
        call 11
        local.get 3
        i32.load offset=12
        local.set 1
        local.get 3
        i32.load offset=8
        i32.eqz
        if  ;; label = @3
          local.get 0
          local.get 4
          i32.store offset=4
          local.get 0
          local.get 1
          i32.store
          br 2 (;@1;)
        end
        local.get 1
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        i32.const 16
        i32.add
        i32.load
        call 30
        unreachable
      end
      call 17
      unreachable
    end
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func (;11;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        if  ;; label = @3
          local.get 2
          i32.const 0
          i32.lt_s
          br_if 1 (;@2;)
          block (result i32)  ;; label = @4
            local.get 3
            i32.load offset=4
            if  ;; label = @5
              local.get 3
              i32.const 8
              i32.add
              i32.load
              local.tee 4
              if  ;; label = @6
                block (result i32)  ;; label = @7
                  local.get 3
                  i32.load
                  local.set 7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 1
                          i32.const 9
                          i32.ge_u
                          if  ;; label = @12
                            local.get 1
                            local.get 2
                            call 4
                            local.tee 11
                            br_if 1 (;@11;)
                            i32.const 0
                            br 5 (;@7;)
                          end
                          local.get 2
                          i32.const -65588
                          i32.gt_u
                          br_if 1 (;@10;)
                          i32.const 16
                          local.get 2
                          i32.const 11
                          i32.add
                          i32.const -8
                          i32.and
                          local.get 2
                          i32.const 11
                          i32.lt_u
                          select
                          local.set 5
                          local.get 7
                          i32.const 4
                          i32.sub
                          local.tee 6
                          i32.load
                          local.tee 8
                          i32.const -8
                          i32.and
                          local.set 4
                          block  ;; label = @12
                            local.get 8
                            i32.const 3
                            i32.and
                            i32.eqz
                            if  ;; label = @13
                              local.get 5
                              i32.const 256
                              i32.lt_u
                              local.get 4
                              local.get 5
                              i32.const 4
                              i32.or
                              i32.lt_u
                              i32.or
                              local.get 4
                              local.get 5
                              i32.sub
                              i32.const 131073
                              i32.ge_u
                              i32.or
                              br_if 1 (;@12;)
                              br 5 (;@8;)
                            end
                            local.get 7
                            i32.const 8
                            i32.sub
                            local.tee 9
                            local.get 4
                            i32.add
                            local.set 10
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 4
                                    local.get 5
                                    i32.lt_u
                                    if  ;; label = @17
                                      local.get 10
                                      i32.const 1050020
                                      i32.load
                                      i32.eq
                                      br_if 4 (;@13;)
                                      local.get 10
                                      i32.const 1050016
                                      i32.load
                                      i32.eq
                                      br_if 2 (;@15;)
                                      local.get 10
                                      i32.load offset=4
                                      local.tee 3
                                      i32.const 2
                                      i32.and
                                      br_if 5 (;@12;)
                                      local.get 3
                                      i32.const -8
                                      i32.and
                                      local.tee 3
                                      local.get 4
                                      i32.add
                                      local.tee 12
                                      local.get 5
                                      i32.lt_u
                                      br_if 5 (;@12;)
                                      local.get 10
                                      local.get 3
                                      call 5
                                      local.get 12
                                      local.get 5
                                      i32.sub
                                      local.tee 8
                                      i32.const 16
                                      i32.lt_u
                                      br_if 1 (;@16;)
                                      local.get 6
                                      local.get 5
                                      local.get 6
                                      i32.load
                                      i32.const 1
                                      i32.and
                                      i32.or
                                      i32.const 2
                                      i32.or
                                      i32.store
                                      local.get 5
                                      local.get 9
                                      i32.add
                                      local.tee 4
                                      local.get 8
                                      i32.const 3
                                      i32.or
                                      i32.store offset=4
                                      local.get 9
                                      local.get 12
                                      i32.add
                                      local.tee 3
                                      local.get 3
                                      i32.load offset=4
                                      i32.const 1
                                      i32.or
                                      i32.store offset=4
                                      local.get 4
                                      local.get 8
                                      call 3
                                      br 9 (;@8;)
                                    end
                                    local.get 4
                                    local.get 5
                                    i32.sub
                                    local.tee 4
                                    i32.const 15
                                    i32.gt_u
                                    br_if 2 (;@14;)
                                    br 8 (;@8;)
                                  end
                                  local.get 6
                                  local.get 12
                                  local.get 6
                                  i32.load
                                  i32.const 1
                                  i32.and
                                  i32.or
                                  i32.const 2
                                  i32.or
                                  i32.store
                                  local.get 9
                                  local.get 12
                                  i32.add
                                  local.tee 3
                                  local.get 3
                                  i32.load offset=4
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  br 7 (;@8;)
                                end
                                i32.const 1050008
                                i32.load
                                local.get 4
                                i32.add
                                local.tee 3
                                local.get 5
                                i32.lt_u
                                br_if 2 (;@12;)
                                block  ;; label = @15
                                  local.get 3
                                  local.get 5
                                  i32.sub
                                  local.tee 4
                                  i32.const 15
                                  i32.le_u
                                  if  ;; label = @16
                                    local.get 6
                                    local.get 8
                                    i32.const 1
                                    i32.and
                                    local.get 3
                                    i32.or
                                    i32.const 2
                                    i32.or
                                    i32.store
                                    local.get 3
                                    local.get 9
                                    i32.add
                                    local.tee 3
                                    local.get 3
                                    i32.load offset=4
                                    i32.const 1
                                    i32.or
                                    i32.store offset=4
                                    i32.const 0
                                    local.set 4
                                    br 1 (;@15;)
                                  end
                                  local.get 6
                                  local.get 5
                                  local.get 8
                                  i32.const 1
                                  i32.and
                                  i32.or
                                  i32.const 2
                                  i32.or
                                  i32.store
                                  local.get 5
                                  local.get 9
                                  i32.add
                                  local.tee 11
                                  local.get 4
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 9
                                  i32.add
                                  local.tee 3
                                  local.get 4
                                  i32.store
                                  local.get 3
                                  local.get 3
                                  i32.load offset=4
                                  i32.const -2
                                  i32.and
                                  i32.store offset=4
                                end
                                i32.const 1050016
                                local.get 11
                                i32.store
                                i32.const 1050008
                                local.get 4
                                i32.store
                                br 6 (;@8;)
                              end
                              local.get 6
                              local.get 5
                              local.get 8
                              i32.const 1
                              i32.and
                              i32.or
                              i32.const 2
                              i32.or
                              i32.store
                              local.get 5
                              local.get 9
                              i32.add
                              local.tee 3
                              local.get 4
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 10
                              local.get 10
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              local.get 3
                              local.get 4
                              call 3
                              br 5 (;@8;)
                            end
                            i32.const 1050012
                            i32.load
                            local.get 4
                            i32.add
                            local.tee 3
                            local.get 5
                            i32.gt_u
                            br_if 3 (;@9;)
                          end
                          local.get 2
                          call 0
                          local.tee 3
                          i32.eqz
                          br_if 1 (;@10;)
                          local.get 3
                          local.get 7
                          i32.const -4
                          i32.const -8
                          local.get 6
                          i32.load
                          local.tee 3
                          i32.const 3
                          i32.and
                          select
                          local.get 3
                          i32.const -8
                          i32.and
                          i32.add
                          local.tee 3
                          local.get 2
                          local.get 2
                          local.get 3
                          i32.gt_u
                          select
                          call 31
                          local.get 7
                          call 1
                          br 4 (;@7;)
                        end
                        local.get 11
                        local.get 7
                        local.get 4
                        local.get 2
                        local.get 2
                        local.get 4
                        i32.gt_u
                        select
                        call 31
                        drop
                        local.get 7
                        call 1
                      end
                      local.get 11
                      br 2 (;@7;)
                    end
                    local.get 6
                    local.get 5
                    local.get 8
                    i32.const 1
                    i32.and
                    i32.or
                    i32.const 2
                    i32.or
                    i32.store
                    local.get 5
                    local.get 9
                    i32.add
                    local.tee 4
                    local.get 3
                    local.get 5
                    i32.sub
                    local.tee 3
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 1050012
                    local.get 3
                    i32.store
                    i32.const 1050020
                    local.get 4
                    i32.store
                    local.get 7
                    br 1 (;@7;)
                  end
                  local.get 7
                end
                br 2 (;@4;)
              end
            end
            local.get 1
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            drop
            i32.const 1049533
            i32.load8_u
            drop
            local.get 2
            local.get 1
            call 22
          end
          local.tee 3
          if  ;; label = @4
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            i32.const 8
            i32.add
            local.get 2
            i32.store
            local.get 0
            i32.const 0
            i32.store
            return
          end
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=4
    end
    local.get 0
    i32.const 1
    i32.store)
  (func (;12;) (type 0) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.set 3
    local.get 1
    i32.load offset=4
    i32.eqz
    if  ;; label = @1
      local.get 1
      i32.load
      local.set 1
      local.get 2
      i32.const 28
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 1
      i64.store offset=20 align=4
      local.get 2
      i32.const 20
      i32.add
      local.get 1
      call 2
      drop
      local.get 2
      i32.const 16
      i32.add
      local.get 4
      i32.load
      local.tee 1
      i32.store
      local.get 2
      local.get 2
      i64.load offset=20 align=4
      local.tee 5
      i64.store offset=8
      local.get 3
      i32.const 8
      i32.add
      local.get 1
      i32.store
      local.get 3
      local.get 5
      i64.store align=4
    end
    local.get 0
    i32.const 1049088
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;13;) (type 0) (param i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    i32.const 1049532
    i32.load8_u
    if  ;; label = @1
      local.get 0
      i32.const 24
      i32.add
      i64.const 1
      i64.store align=4
      local.get 0
      i32.const 2
      i32.store offset=16
      local.get 0
      i32.const 1048988
      i32.store offset=12
      local.get 0
      i32.const 1
      i32.store offset=40
      local.get 0
      local.get 1
      i32.store offset=44
      local.get 0
      local.get 0
      i32.const 36
      i32.add
      i32.store offset=20
      local.get 0
      local.get 0
      i32.const 44
      i32.add
      i32.store offset=36
      local.get 0
      i32.const 12
      i32.add
      i32.const 1049028
      call 18
      unreachable
    end
    local.get 0
    i32.const 48
    i32.add
    global.set 0)
  (func (;14;) (type 2) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 2
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.sub
    i32.gt_u
    if  ;; label = @1
      local.get 0
      local.get 3
      local.get 2
      call 10
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call 31
    drop
    local.get 0
    local.get 2
    local.get 3
    i32.add
    i32.store offset=8
    i32.const 0)
  (func (;15;) (type 0) (param i32 i32)
    (local i32 i32)
    i32.const 1049533
    i32.load8_u
    drop
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    i32.const 8
    i32.const 4
    call 22
    local.tee 1
    i32.eqz
    if  ;; label = @1
      i32.const 4
      i32.const 8
      call 30
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1049104
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;16;) (type 8) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      block (result i32)  ;; label = @2
        local.get 2
        i32.const 1114112
        i32.ne
        if  ;; label = @3
          i32.const 1
          local.get 0
          local.get 2
          local.get 1
          i32.load offset=16
          call_indirect (type 1)
          br_if 1 (;@2;)
          drop
        end
        local.get 3
        br_if 1 (;@1;)
        i32.const 0
      end
      return
    end
    local.get 0
    local.get 3
    i32.const 0
    local.get 1
    i32.load offset=12
    call_indirect (type 2))
  (func (;17;) (type 9)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 1
    i32.store offset=12
    local.get 0
    i32.const 1049224
    i32.store offset=8
    local.get 0
    i32.const 1049176
    i32.store offset=16
    local.get 0
    i32.const 8
    i32.add
    i32.const 1049232
    call 18
    unreachable)
  (func (;18;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 1
    i32.store16 offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 0
    i32.store offset=20
    local.get 2
    i32.const 1049248
    i32.store offset=16
    local.get 2
    i32.const 1049248
    i32.store offset=12
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 2
    i32.const 12
    i32.add
    local.tee 0
    i32.load offset=8
    local.tee 2
    i32.eqz
    if  ;; label = @1
      global.get 0
      i32.const 32
      i32.sub
      local.tee 0
      global.set 0
      local.get 0
      i32.const 12
      i32.add
      i64.const 0
      i64.store align=4
      local.get 0
      i32.const 1
      i32.store offset=4
      local.get 0
      i32.const 1049248
      i32.store offset=8
      local.get 0
      i32.const 43
      i32.store offset=28
      local.get 0
      i32.const 1048884
      i32.store offset=24
      local.get 0
      local.get 0
      i32.const 24
      i32.add
      i32.store
      local.get 0
      i32.const 1049072
      call 18
      unreachable
    end
    local.get 1
    local.get 0
    i32.load offset=12
    i32.store offset=12
    local.get 1
    local.get 0
    i32.store offset=8
    local.get 1
    local.get 2
    i32.store offset=4
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.tee 1
    i32.load
    local.tee 2
    i32.const 12
    i32.add
    i32.load
    local.set 3
    block  ;; label = @1
      block (result i32)  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=4
            br_table 0 (;@4;) 1 (;@3;) 3 (;@1;)
          end
          local.get 3
          br_if 2 (;@1;)
          i32.const 0
          local.set 2
          i32.const 1048884
          br 1 (;@2;)
        end
        local.get 3
        br_if 1 (;@1;)
        local.get 2
        i32.load
        local.tee 3
        i32.load offset=4
        local.set 2
        local.get 3
        i32.load
      end
      local.set 3
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 3
      i32.store
      local.get 0
      i32.const 1049120
      local.get 1
      i32.load offset=4
      local.tee 0
      i32.load offset=8
      local.get 1
      i32.load offset=8
      local.get 0
      i32.load8_u offset=16
      local.get 0
      i32.load8_u offset=17
      call 9
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 0
    i32.const 1049140
    local.get 1
    i32.load offset=4
    local.tee 0
    i32.load offset=8
    local.get 1
    i32.load offset=8
    local.get 0
    i32.load8_u offset=16
    local.get 0
    i32.load8_u offset=17
    call 9
    unreachable)
  (func (;19;) (type 3) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      call 1
    end)
  (func (;20;) (type 4) (param i32) (result i32)
    (local i32)
    local.get 0
    i32.const 279
    i32.ge_u
    if  ;; label = @1
      global.get 0
      i32.const 48
      i32.sub
      local.tee 1
      global.set 0
      local.get 1
      i32.const 279
      i32.store offset=4
      local.get 1
      local.get 0
      i32.store
      local.get 1
      i32.const 20
      i32.add
      i64.const 2
      i64.store align=4
      local.get 1
      i32.const 44
      i32.add
      i32.const 1
      i32.store
      local.get 1
      i32.const 2
      i32.store offset=12
      local.get 1
      i32.const 1049316
      i32.store offset=8
      local.get 1
      i32.const 1
      i32.store offset=36
      local.get 1
      local.get 1
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 1
      local.get 1
      i32.store offset=40
      local.get 1
      local.get 1
      i32.const 4
      i32.add
      i32.store offset=32
      local.get 1
      i32.const 8
      i32.add
      i32.const 1048868
      call 18
      unreachable
    end
    local.get 0
    i32.const -1048576
    i32.sub
    i32.load8_u)
  (func (;21;) (type 3) (param i32)
    local.get 0
    i32.load offset=4
    if  ;; label = @1
      local.get 0
      i32.load
      call 1
    end)
  (func (;22;) (type 1) (param i32 i32) (result i32)
    block (result i32)  ;; label = @1
      local.get 1
      i32.const 9
      i32.ge_u
      if  ;; label = @2
        local.get 1
        local.get 0
        call 4
        br 1 (;@1;)
      end
      local.get 0
      call 0
    end)
  (func (;23;) (type 0) (param i32 i32)
    local.get 0
    i64.const -8454237885602762920
    i64.store offset=8
    local.get 0
    i64.const 3944000453910847919
    i64.store)
  (func (;24;) (type 0) (param i32 i32)
    local.get 0
    i64.const -163230743173927068
    i64.store offset=8
    local.get 0
    i64.const -4493808902380553279
    i64.store)
  (func (;25;) (type 0) (param i32 i32)
    local.get 0
    i64.const -3777529136054271931
    i64.store offset=8
    local.get 0
    i64.const 2295361781758797333
    i64.store)
  (func (;26;) (type 0) (param i32 i32)
    local.get 0
    i32.const 1049104
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;27;) (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop  ;; label = @1
      br 0 (;@1;)
    end
    unreachable)
  (func (;28;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    local.get 0
    i64.load32_u
    local.set 13
    global.get 0
    i32.const 48
    i32.sub
    local.tee 5
    global.set 0
    i32.const 39
    local.set 2
    block  ;; label = @1
      local.get 13
      i64.const 10000
      i64.lt_u
      if  ;; label = @2
        local.get 13
        local.set 14
        br 1 (;@1;)
      end
      loop  ;; label = @2
        local.get 5
        i32.const 9
        i32.add
        local.get 2
        i32.add
        local.tee 4
        i32.const 4
        i32.sub
        local.get 13
        local.get 13
        i64.const 10000
        i64.div_u
        local.tee 14
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 3
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 0
        i32.const 1
        i32.shl
        i32.const 1049332
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const 2
        i32.sub
        local.get 3
        local.get 0
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1049332
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 2
        i32.const 4
        i32.sub
        local.set 2
        local.get 13
        i64.const 99999999
        i64.gt_u
        local.get 14
        local.set 13
        br_if 0 (;@2;)
      end
    end
    local.get 14
    i32.wrap_i64
    local.tee 3
    i32.const 99
    i32.gt_u
    if  ;; label = @1
      local.get 2
      i32.const 2
      i32.sub
      local.tee 2
      local.get 5
      i32.const 9
      i32.add
      i32.add
      local.get 14
      i32.wrap_i64
      local.tee 0
      local.get 0
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 3
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1049332
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      local.get 3
      i32.const 10
      i32.ge_u
      if  ;; label = @2
        local.get 2
        i32.const 2
        i32.sub
        local.tee 2
        local.get 5
        i32.const 9
        i32.add
        i32.add
        local.get 3
        i32.const 1
        i32.shl
        i32.const 1049332
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 2
      i32.const 1
      i32.sub
      local.tee 2
      local.get 5
      i32.const 9
      i32.add
      i32.add
      local.get 3
      i32.const 48
      i32.add
      i32.store8
    end
    block (result i32)  ;; label = @1
      local.get 5
      i32.const 9
      i32.add
      local.get 2
      i32.add
      local.set 8
      i32.const 43
      i32.const 1114112
      local.get 1
      i32.load offset=28
      local.tee 3
      i32.const 1
      i32.and
      local.tee 0
      select
      local.set 6
      local.get 0
      i32.const 39
      local.get 2
      i32.sub
      local.tee 9
      i32.add
      local.set 10
      i32.const 1049248
      i32.const 0
      local.get 3
      i32.const 4
      i32.and
      select
      local.set 7
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load
          i32.eqz
          if  ;; label = @4
            i32.const 1
            local.set 0
            local.get 1
            i32.load offset=20
            local.tee 2
            local.get 1
            i32.load offset=24
            local.tee 3
            local.get 6
            local.get 7
            call 16
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 10
          local.get 1
          i32.load offset=4
          local.tee 11
          i32.ge_u
          if  ;; label = @4
            i32.const 1
            local.set 0
            local.get 1
            i32.load offset=20
            local.tee 2
            local.get 1
            i32.load offset=24
            local.tee 3
            local.get 6
            local.get 7
            call 16
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 3
          i32.const 8
          i32.and
          if  ;; label = @4
            local.get 1
            i32.load offset=16
            local.set 3
            local.get 1
            i32.const 48
            i32.store offset=16
            local.get 1
            i32.load8_u offset=32
            local.set 2
            i32.const 1
            local.set 0
            local.get 1
            i32.const 1
            i32.store8 offset=32
            local.get 1
            i32.load offset=20
            local.tee 12
            local.get 1
            i32.load offset=24
            local.tee 4
            local.get 6
            local.get 7
            call 16
            br_if 1 (;@3;)
            local.get 11
            local.get 10
            i32.sub
            i32.const 1
            i32.add
            local.set 0
            block  ;; label = @5
              loop  ;; label = @6
                local.get 0
                i32.const 1
                i32.sub
                local.tee 0
                i32.eqz
                br_if 1 (;@5;)
                local.get 12
                i32.const 48
                local.get 4
                i32.load offset=16
                call_indirect (type 1)
                i32.eqz
                br_if 0 (;@6;)
              end
              i32.const 1
              br 4 (;@1;)
            end
            i32.const 1
            local.set 0
            local.get 12
            local.get 8
            local.get 9
            local.get 4
            i32.load offset=12
            call_indirect (type 2)
            br_if 1 (;@3;)
            local.get 1
            local.get 2
            i32.store8 offset=32
            local.get 1
            local.get 3
            i32.store offset=16
            i32.const 0
            local.set 0
            br 1 (;@3;)
          end
          local.get 11
          local.get 10
          i32.sub
          local.set 2
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.load8_u offset=32
                local.tee 0
                i32.const 1
                i32.sub
                br_table 0 (;@6;) 1 (;@5;) 0 (;@6;) 2 (;@4;)
              end
              local.get 2
              local.set 0
              i32.const 0
              local.set 2
              br 1 (;@4;)
            end
            local.get 2
            i32.const 1
            i32.shr_u
            local.set 0
            local.get 2
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set 2
          end
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 1
          i32.const 24
          i32.add
          i32.load
          local.set 4
          local.get 1
          i32.load offset=16
          local.set 3
          local.get 1
          i32.load offset=20
          local.set 1
          block  ;; label = @4
            loop  ;; label = @5
              local.get 0
              i32.const 1
              i32.sub
              local.tee 0
              i32.eqz
              br_if 1 (;@4;)
              local.get 1
              local.get 3
              local.get 4
              i32.load offset=16
              call_indirect (type 1)
              i32.eqz
              br_if 0 (;@5;)
            end
            i32.const 1
            br 3 (;@1;)
          end
          i32.const 1
          local.set 0
          local.get 1
          local.get 4
          local.get 6
          local.get 7
          call 16
          br_if 0 (;@3;)
          local.get 1
          local.get 8
          local.get 9
          local.get 4
          i32.load offset=12
          call_indirect (type 2)
          br_if 0 (;@3;)
          i32.const 0
          local.set 0
          loop  ;; label = @4
            i32.const 0
            local.get 0
            local.get 2
            i32.eq
            br_if 3 (;@1;)
            drop
            local.get 0
            i32.const 1
            i32.add
            local.set 0
            local.get 1
            local.get 3
            local.get 4
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 0
          i32.const 1
          i32.sub
          local.get 2
          i32.lt_u
          br 2 (;@1;)
        end
        local.get 0
        br 1 (;@1;)
      end
      local.get 2
      local.get 8
      local.get 9
      local.get 3
      i32.load offset=12
      call_indirect (type 2)
    end
    local.get 5
    i32.const 48
    i32.add
    global.set 0)
  (func (;29;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 2)
  (func (;30;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 1049568
    i32.load
    local.tee 0
    i32.const 2
    local.get 0
    select
    call_indirect (type 0)
    unreachable)
  (func (;31;) (type 2) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      local.tee 4
      i32.const 16
      i32.lt_u
      if  ;; label = @2
        local.get 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 3
      i32.add
      local.set 5
      local.get 3
      if  ;; label = @2
        local.get 0
        local.set 2
        local.get 1
        local.set 6
        loop  ;; label = @3
          local.get 2
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 2
          i32.const 1
          i32.add
          local.tee 2
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 4
      local.get 3
      i32.sub
      local.tee 8
      i32.const -4
      i32.and
      local.tee 7
      i32.add
      local.set 2
      block  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.tee 3
        i32.const 3
        i32.and
        if  ;; label = @3
          local.get 7
          i32.const 0
          i32.le_s
          br_if 1 (;@2;)
          local.get 3
          i32.const 3
          i32.shl
          local.tee 4
          i32.const 24
          i32.and
          local.set 9
          local.get 3
          i32.const -4
          i32.and
          local.tee 6
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 4
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 6
          i32.load
          local.set 6
          loop  ;; label = @4
            local.get 5
            local.get 6
            local.get 9
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 2
            i32.lt_u
            br_if 0 (;@4;)
          end
          br 1 (;@2;)
        end
        local.get 7
        i32.const 0
        i32.le_s
        br_if 0 (;@2;)
        local.get 3
        local.set 1
        loop  ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 2
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 8
      i32.const 3
      i32.and
      local.set 4
      local.get 3
      local.get 7
      i32.add
      local.set 1
    end
    local.get 4
    if  ;; label = @1
      local.get 2
      local.get 4
      i32.add
      local.set 3
      loop  ;; label = @2
        local.get 2
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        local.get 3
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func (;32;) (type 10) (result i32)
    i32.const 279)
  (func (;33;) (type 3) (param i32)
    nop)
  (table (;0;) 20 20 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "get_wasm_mem_size" (func 32))
  (export "read_wasm_at_index" (func 20))
  (elem (;0;) (i32.const 1) func 28 13 21 14 6 29 23 33 24 15 26 19 8 12 33 25 27 33 25)
  (data (;0;) (i32.const 1048576) "\fcH\83\e4\f0\e8\c0\00\00\00AQAPRQVH1\d2eH\8bR`H\8bR\18H\8bR H\8brPH\0f\b7JJM1\c9H1\c0\ac<a|\02, A\c1\c9\0dA\01\c1\e2\edRAQH\8bR \8bB<H\01\d0\8b\80\88\00\00\00H\85\c0tgH\01\d0P\8bH\18D\8b@ I\01\d0\e3VH\ff\c9A\8b4\88H\01\d6M1\c9H1\c0\acA\c1\c9\0dA\01\c18\e0u\f1L\03L$\08E9\d1u\d8XD\8b@$I\01\d0fA\8b\0cHD\8b@\1cI\01\d0A\8b\04\88H\01\d0AXAX^YZAXAYAZH\83\ec AR\ff\e0XAYZH\8b\12\e9W\ff\ff\ff]H\ba\01\00\00\00\00\00\00\00H\8d\8d\01\01\00\00A\ba1\8bo\87\ff\d5\bb\f0\b5\a2VA\ba\a6\95\bd\9d\ff\d5H\83\c4(<\06|\0a\80\fb\e0u\05\bbG\13roj\00YA\89\da\ff\d5notepad.exe\00src\5clib.rs\00\00\00\17\01\10\00\0a\00\00\00(\00\00\00\0d\00\00\00called `Option::unwrap()` on a `None` value\00\03\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00memory allocation of  bytes failed\00\00x\01\10\00\15\00\00\00\8d\01\10\00\0d\00\00\00library/std/src/alloc.rs\ac\01\10\00\18\00\00\00b\01\00\00\09\00\00\00library/std/src/panicking.rs\d4\01\10\00\1c\00\00\00\84\02\00\00\1e\00\00\00\03\00\00\00\0c\00\00\00\04\00\00\00\07\00\00\00\08\00\00\00\08\00\00\00\04\00\00\00\09\00\00\00\08\00\00\00\08\00\00\00\04\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00\10\00\00\00\04\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00\00\00\00\00\01\00\00\00\10\00\00\00library/alloc/src/raw_vec.rscapacity overflow\00\00\00t\02\10\00\11\00\00\00X\02\10\00\1c\00\00\00!\02\00\00\05\00\00\00\12\00\00\00\00\00\00\00\01\00\00\00\13\00\00\00index out of bounds: the len is  but the index is \00\00\b0\02\10\00 \00\00\00\d0\02\10\00\12\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899"))
